/*
 * Copyright (C) 2016, The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#include "generate_java.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <algorithm>
#include <map>
#include <memory>
#include <sstream>

#include <android-base/stringprintf.h>

#include "aidl_to_java.h"
#include "code_writer.h"
#include "logging.h"

using ::android::base::EndsWith;
using ::android::base::Join;
using ::android::base::StartsWith;
using std::string;
using std::unique_ptr;

namespace {
// join two non-empty strings according to `camelCase` naming.
inline string camelcase_join(const string& a, const string& b, const AidlNode& context) {
  AIDL_FATAL_IF(b.size() <= 0 || a.size() <= 0, context) << "Name cannot be empty.";
  std::string name = a + b;
  name[a.size()] = static_cast<char>(toupper(name[a.size()]));
  return name;
}
inline string getter_name(const AidlVariableDeclaration& variable) {
  return camelcase_join("get", variable.GetName(), variable);
}
inline string setter_name(const AidlVariableDeclaration& variable) {
  return camelcase_join("set", variable.GetName(), variable);
}
}  // namespace

namespace android {
namespace aidl {
namespace java {

bool generate_java_interface(const string& filename, const AidlInterface* iface,
                             const AidlTypenames& typenames, const IoDelegate& io_delegate,
                             const Options& options) {
  auto cl = generate_binder_interface_class(iface, typenames, options);

  std::unique_ptr<Document> document =
      std::make_unique<Document>("" /* no comment */, iface->GetPackage(), std::move(cl));

  CodeWriterPtr code_writer = io_delegate.GetCodeWriter(filename);
  document->Write(code_writer.get());

  return true;
}

bool generate_java_parcel(const std::string& filename, const AidlStructuredParcelable* parcel,
                          const AidlTypenames& typenames, const IoDelegate& io_delegate) {
  auto cl = generate_parcel_class(parcel, typenames);

  std::unique_ptr<Document> document =
      std::make_unique<Document>("" /* no comment */, parcel->GetPackage(), std::move(cl));

  CodeWriterPtr code_writer = io_delegate.GetCodeWriter(filename);
  document->Write(code_writer.get());

  return true;
}

bool generate_java_enum_declaration(const std::string& filename,
                                    const AidlEnumDeclaration* enum_decl,
                                    const AidlTypenames& typenames, const IoDelegate& io_delegate) {
  CodeWriterPtr code_writer = io_delegate.GetCodeWriter(filename);
  generate_enum(code_writer, enum_decl, typenames);
  return true;
}

bool generate_java_union_declaration(const std::string& filename, const AidlUnionDecl* decl,
                                     const AidlTypenames& typenames,
                                     const IoDelegate& io_delegate) {
  CodeWriterPtr code_writer = io_delegate.GetCodeWriter(filename);
  generate_union(*code_writer, decl, typenames);
  return true;
}

bool generate_java(const std::string& filename, const AidlDefinedType* defined_type,
                   const AidlTypenames& typenames, const IoDelegate& io_delegate,
                   const Options& options) {
  if (const AidlStructuredParcelable* parcelable = defined_type->AsStructuredParcelable();
      parcelable != nullptr) {
    return generate_java_parcel(filename, parcelable, typenames, io_delegate);
  }

  if (const AidlEnumDeclaration* enum_decl = defined_type->AsEnumDeclaration();
      enum_decl != nullptr) {
    return generate_java_enum_declaration(filename, enum_decl, typenames, io_delegate);
  }

  if (const AidlInterface* interface = defined_type->AsInterface(); interface != nullptr) {
    return generate_java_interface(filename, interface, typenames, io_delegate, options);
  }

  if (const AidlUnionDecl* union_decl = defined_type->AsUnionDeclaration(); union_decl != nullptr) {
    return generate_java_union_declaration(filename, union_decl, typenames, io_delegate);
  }

  AIDL_FATAL(defined_type) << "Unrecognized type sent for Java generation.";
  return false;
}

std::unique_ptr<android::aidl::java::Class> generate_parcel_class(
    const AidlStructuredParcelable* parcel, const AidlTypenames& typenames) {
  auto parcel_class = std::make_unique<Class>();
  parcel_class->comment = parcel->GetComments();
  parcel_class->modifiers = PUBLIC;
  parcel_class->what = Class::CLASS;
  parcel_class->type = parcel->GetCanonicalName();
  parcel_class->interfaces.push_back("android.os.Parcelable");
  parcel_class->annotations = generate_java_annotations(*parcel);

  if (parcel->IsGeneric()) {
    parcel_class->type += "<" + base::Join(parcel->GetTypeParameters(), ",") + ">";
  }

  for (const auto& variable : parcel->GetFields()) {
    std::ostringstream out;
    out << variable->GetType().GetComments() << "\n";
    for (const auto& a : generate_java_annotations(variable->GetType())) {
      out << a << "\n";
    }
    out << "public ";

    if (variable->GetType().GetName() == "ParcelableHolder" || parcel->IsJavaOnlyImmutable()) {
      out << "final ";
    }
    out << JavaSignatureOf(variable->GetType(), typenames) << " " << variable->GetName();
    if (!parcel->IsJavaOnlyImmutable() && variable->GetDefaultValue()) {
      out << " = " << variable->ValueString(ConstantValueDecorator);
    } else if (variable->GetType().GetName() == "ParcelableHolder") {
      out << std::boolalpha;
      out << " = new " << JavaSignatureOf(variable->GetType(), typenames) << "(";
      if (parcel->IsVintfStability()) {
        out << "android.os.Parcelable.PARCELABLE_STABILITY_VINTF";
      } else {
        out << "android.os.Parcelable.PARCELABLE_STABILITY_LOCAL";
      }
      out << ")";
      out << std::noboolalpha;
    }
    out << ";\n";
    parcel_class->elements.push_back(std::make_shared<LiteralClassElement>(out.str()));
  }

  std::ostringstream out;
  if (parcel->IsJavaOnlyImmutable()) {
    auto builder_class = std::make_shared<Class>();
    builder_class->modifiers = PUBLIC | FINAL | STATIC;
    builder_class->what = Class::CLASS;
    builder_class->type = "Builder";

    out.str("");
    for (const auto& variable : parcel->GetFields()) {
      out << "private " << JavaSignatureOf(variable->GetType(), typenames) << " "
          << variable->GetName();
      if (variable->GetDefaultValue()) {
        out << " = " << variable->ValueString(ConstantValueDecorator);
      }
      out << ";\n";
      out << "public Builder " << setter_name(*variable) << "("
          << JavaSignatureOf(variable->GetType(), typenames) << " " << variable->GetName()
          << ") {\n"
          << "  "
          << "this." << variable->GetName() << " = " << variable->GetName() << ";\n"
          << "  return this;\n"
          << "}\n";
    }
    out << "public " << parcel->GetCanonicalName() << " build() {\n"
        << "  return new " << parcel->GetCanonicalName() << "(";
    std::vector<std::string> variables;
    std::transform(parcel->GetFields().begin(), parcel->GetFields().end(),
                   std::back_inserter(variables), [](const auto& f) { return f->GetName(); });
    out << base::Join(variables, ", ") << ");\n"
        << "}\n";
    builder_class->elements.push_back(std::make_shared<LiteralClassElement>(out.str()));
    parcel_class->elements.push_back(builder_class);
  }
  if (parcel->IsVintfStability()) {
    parcel_class->elements.push_back(std::make_shared<LiteralClassElement>(
        "@Override\n public final int getStability() { return "
        "android.os.Parcelable.PARCELABLE_STABILITY_VINTF; }\n"));
  }

  out.str("");
  out << "public static final android.os.Parcelable.Creator<" << parcel->GetName() << "> CREATOR = "
      << "new android.os.Parcelable.Creator<" << parcel->GetName() << ">() {\n";
  out << "  @Override\n";
  out << "  public " << parcel->GetName()
      << " createFromParcel(android.os.Parcel _aidl_source) {\n";
  if (parcel->IsJavaOnlyImmutable()) {
    out << "    return internalCreateFromParcel(_aidl_source);\n";
  } else {
    out << "    " << parcel->GetName() << " _aidl_out = new " << parcel->GetName() << "();\n";
    out << "    _aidl_out.readFromParcel(_aidl_source);\n";
    out << "    return _aidl_out;\n";
  }
  out << "  }\n";
  out << "  @Override\n";
  out << "  public " << parcel->GetName() << "[] newArray(int _aidl_size) {\n";
  out << "    return new " << parcel->GetName() << "[_aidl_size];\n";
  out << "  }\n";
  out << "};\n";
  parcel_class->elements.push_back(std::make_shared<LiteralClassElement>(out.str()));

  auto flag_variable = std::make_shared<Variable>("int", "_aidl_flag");
  auto parcel_variable = std::make_shared<Variable>("android.os.Parcel", "_aidl_parcel");

  auto write_method = std::make_shared<Method>();
  write_method->modifiers = PUBLIC | OVERRIDE | FINAL;
  write_method->returnType = "void";
  write_method->name = "writeToParcel";
  write_method->parameters.push_back(parcel_variable);
  write_method->parameters.push_back(flag_variable);
  write_method->statements = std::make_shared<StatementBlock>();

  out.str("");
  out << "int _aidl_start_pos = _aidl_parcel.dataPosition();\n"
      << "_aidl_parcel.writeInt(0);\n";
  write_method->statements->Add(std::make_shared<LiteralStatement>(out.str()));

  for (const auto& field : parcel->GetFields()) {
    string code;
    CodeWriterPtr writer = CodeWriter::ForString(&code);
    CodeGeneratorContext context{
        .writer = *(writer.get()),
        .typenames = typenames,
        .type = field->GetType(),
        .parcel = parcel_variable->name,
        .var = field->GetName(),
        .is_return_value = false,
    };
    WriteToParcelFor(context);
    writer->Close();
    write_method->statements->Add(std::make_shared<LiteralStatement>(code));
  }

  out.str("");
  out << "int _aidl_end_pos = _aidl_parcel.dataPosition();\n"
      << "_aidl_parcel.setDataPosition(_aidl_start_pos);\n"
      << "_aidl_parcel.writeInt(_aidl_end_pos - _aidl_start_pos);\n"
      << "_aidl_parcel.setDataPosition(_aidl_end_pos);\n";

  write_method->statements->Add(std::make_shared<LiteralStatement>(out.str()));

  parcel_class->elements.push_back(write_method);

  if (parcel->IsJavaOnlyImmutable()) {
    auto constructor = std::make_shared<Method>();
    constructor->modifiers = PUBLIC;
    constructor->name = parcel->GetName();
    constructor->statements = std::make_shared<StatementBlock>();
    for (const auto& field : parcel->GetFields()) {
      constructor->parameters.push_back(std::make_shared<Variable>(
          JavaSignatureOf(field->GetType(), typenames), field->GetName()));
      out.str("");

      out << "this." << field->GetName() << " = ";
      if (field->GetType().GetName() == "List") {
        out << field->GetName() << " == null ? null : java.util.Collections.unmodifiableList("
            << field->GetName() << ");\n";
      } else if (field->GetType().GetName() == "Map") {
        out << field->GetName() << " == null ? null : java.util.Collections.unmodifiableMap("
            << field->GetName() << ");\n";
      } else {
        out << field->GetName() << ";\n";
      }
      constructor->statements->Add(std::make_shared<LiteralStatement>(out.str()));
    }
    parcel_class->elements.push_back(constructor);
  }

  // For an immutable parcelable, generate internalCreateFromParcel method.
  // Otherwise, generate readFromParcel method.
  auto read_or_create_method = std::make_shared<Method>();
  if (parcel->IsJavaOnlyImmutable()) {
    auto constructor = std::make_shared<Method>();
    read_or_create_method->modifiers = PRIVATE | STATIC;
    read_or_create_method->returnType = parcel->GetName();
    read_or_create_method->name = "internalCreateFromParcel";
    read_or_create_method->parameters.push_back(parcel_variable);
    read_or_create_method->statements = std::make_shared<StatementBlock>();
  } else {
    read_or_create_method->modifiers = PUBLIC | FINAL;
    read_or_create_method->returnType = "void";
    read_or_create_method->name = "readFromParcel";
    read_or_create_method->parameters.push_back(parcel_variable);
    read_or_create_method->statements = std::make_shared<StatementBlock>();
  }
  out.str("");
  const string builder_variable = "_aidl_parcelable_builder";
  if (parcel->IsJavaOnlyImmutable()) {
    out << "Builder " << builder_variable << " = new Builder();\n";
  }
  out << "int _aidl_start_pos = _aidl_parcel.dataPosition();\n"
      << "int _aidl_parcelable_size = _aidl_parcel.readInt();\n"
      << "try {\n"
      << "  if (_aidl_parcelable_size < 0) return";
  if (parcel->IsJavaOnlyImmutable()) {
    out << " " << builder_variable << ".build()";
  }
  out << ";\n";

  read_or_create_method->statements->Add(std::make_shared<LiteralStatement>(out.str()));

  out.str("");
  out << "  if (_aidl_parcel.dataPosition() - _aidl_start_pos >= _aidl_parcelable_size) return";
  if (parcel->IsJavaOnlyImmutable()) {
    out << " " << builder_variable << ".build()";
  }
  out << ";\n";

  std::shared_ptr<LiteralStatement> sizeCheck = nullptr;
  // keep this across different fields in order to create the classloader
  // at most once.
  bool is_classloader_created = false;
  for (const auto& field : parcel->GetFields()) {
    const auto field_variable_name =
        (parcel->IsJavaOnlyImmutable() ? "_aidl_temp_" : "") + field->GetName();
    string code;
    CodeWriterPtr writer = CodeWriter::ForString(&code);
    CodeGeneratorContext context{
        .writer = *(writer.get()),
        .typenames = typenames,
        .type = field->GetType(),
        .parcel = parcel_variable->name,
        .var = field_variable_name,
        .is_classloader_created = &is_classloader_created,
    };
    context.writer.Indent();
    if (parcel->IsJavaOnlyImmutable()) {
      context.writer.Write("%s %s;\n", JavaSignatureOf(field->GetType(), typenames).c_str(),
                           field_variable_name.c_str());
    }
    CreateFromParcelFor(context);
    if (parcel->IsJavaOnlyImmutable()) {
      context.writer.Write("%s.%s(%s);\n", builder_variable.c_str(), setter_name(*field).c_str(),
                           field_variable_name.c_str());
    }
    writer->Close();
    read_or_create_method->statements->Add(std::make_shared<LiteralStatement>(code));
    if (!sizeCheck) sizeCheck = std::make_shared<LiteralStatement>(out.str());
    read_or_create_method->statements->Add(sizeCheck);
  }

  out.str("");
  out << "} finally {\n"
      << "  if (_aidl_start_pos > (Integer.MAX_VALUE - _aidl_parcelable_size)) {\n"
      << "    throw new android.os.BadParcelableException(\"Overflow in the size of "
         "parcelable\");\n"
      << "  }\n"
      << "  _aidl_parcel.setDataPosition(_aidl_start_pos + _aidl_parcelable_size);\n";
  if (parcel->IsJavaOnlyImmutable()) {
    out << "  return " << builder_variable << ".build();\n";
  }
  out << "}\n";

  read_or_create_method->statements->Add(std::make_shared<LiteralStatement>(out.str()));

  parcel_class->elements.push_back(read_or_create_method);

  if (parcel->IsJavaDebug()) {
    out.str("");
    out << "@Override\n";
    out << "public String toString() {\n";
    out << "  java.util.StringJoiner _aidl_sj = new java.util.StringJoiner(";
    out << "\", \", \"{\", \"}\");\n";
    for (const auto& field : parcel->GetFields()) {
      std::string code;
      CodeWriterPtr writer = CodeWriter::ForString(&code);
      CodeGeneratorContext context{
          .writer = *(writer.get()),
          .typenames = typenames,
          .type = field->GetType(),
          .parcel = parcel_variable->name,
          .var = field->GetName(),
          .is_classloader_created = &is_classloader_created,
      };
      ToStringFor(context);
      writer->Close();
      out << "  _aidl_sj.add(\"" << field->GetName() << ": \" + (" << code << "));\n";
    }
    out << "  return \"" << parcel->GetCanonicalName() << "\" + _aidl_sj.toString()  ;\n";
    out << "}\n";
    parcel_class->elements.push_back(std::make_shared<LiteralClassElement>(out.str()));
  }

  auto describe_contents_method = std::make_shared<Method>();
  describe_contents_method->modifiers = PUBLIC | OVERRIDE;
  describe_contents_method->returnType = "int";
  describe_contents_method->name = "describeContents";
  describe_contents_method->statements = std::make_shared<StatementBlock>();
  describe_contents_method->statements->Add(std::make_shared<LiteralStatement>("return 0;\n"));
  parcel_class->elements.push_back(describe_contents_method);

  return parcel_class;
}

void generate_enum(const CodeWriterPtr& code_writer, const AidlEnumDeclaration* enum_decl,
                   const AidlTypenames& typenames) {
  code_writer->Write(
      "/*\n"
      " * This file is auto-generated.  DO NOT MODIFY.\n"
      " */\n");

  code_writer->Write("package %s;\n", enum_decl->GetPackage().c_str());
  code_writer->Write("%s\n", enum_decl->GetComments().c_str());
  for (const std::string& annotation : generate_java_annotations(*enum_decl)) {
    code_writer->Write("%s\n", annotation.c_str());
  }
  code_writer->Write("public @interface %s {\n", enum_decl->GetName().c_str());
  code_writer->Indent();
  for (const auto& enumerator : enum_decl->GetEnumerators()) {
    code_writer->Write("%s", enumerator->GetComments().c_str());
    code_writer->Write(
        "public static final %s %s = %s;\n",
        JavaSignatureOf(enum_decl->GetBackingType(), typenames).c_str(),
        enumerator->GetName().c_str(),
        enumerator->ValueString(enum_decl->GetBackingType(), ConstantValueDecorator).c_str());
  }
  code_writer->Dedent();
  code_writer->Write("}\n");
}

void generate_union(CodeWriter& out, const AidlUnionDecl* decl, const AidlTypenames& typenames) {
  const string tag_type = "int";
  const AidlTypeSpecifier tag_type_specifier(AIDL_LOCATION_HERE, tag_type, false /* isArray */,
                                             nullptr /* type_params */, "");
  const string tag_name = "_tag";
  const string value_name = "_value";
  const string clazz = decl->GetName();

  out << "/*\n";
  out << " * This file is auto-generated.  DO NOT MODIFY.\n";
  out << " */\n";

  out << "package " + decl->GetPackage() + ";\n";
  out << "\n";
  out << decl->GetComments() << "\n";
  for (const auto& annotation : generate_java_annotations(*decl)) {
    out << annotation << "\n";
  }

  out << "public final class " + clazz + " implements android.os.Parcelable {\n";
  out.Indent();

  size_t tag_index = 0;
  out << "// tags union fields\n";
  for (const auto& variable : decl->GetFields()) {
    auto raw_type = variable->GetType().ToString();
    out << "public final static " + tag_type + " " + variable->GetName() + " = " +
               std::to_string(tag_index++) + ";  // " + raw_type + "\n";
  }
  out << "\n";

  out << "private " + tag_type + " " + tag_name + ";\n";
  out << "private Object " + value_name + ";\n";
  out << "\n";

  AIDL_FATAL_IF(decl->GetFields().empty(), *decl) << "Union '" << clazz << "' is empty.";
  const auto& first_field = decl->GetFields()[0];
  const auto& first_type = JavaSignatureOf(first_field->GetType(), typenames);
  const auto& first_value = first_field->ValueString(ConstantValueDecorator);

  // default ctor() inits with first member's default value
  out << "public " + clazz + "() {\n";
  out.Indent();
  out << first_type + " value = " << (first_value.empty() ? "null" : first_value) << ";\n";
  out << "_set(" + first_field->GetName() + ", value);\n";
  out.Dedent();
  out << "}\n";
  // ctor(Parcel)
  out << "private " + clazz + "(android.os.Parcel _aidl_parcel) {\n";
  out << "  readFromParcel(_aidl_parcel);\n";
  out << "}\n";
  // ctor(tag, value)
  out << "private " + clazz + "(" + tag_type + " tag, Object value) {\n";
  out << "  _set(tag, value);\n";
  out << "}\n";
  out << "\n";

  // getTag()
  out << "public " + tag_type + " " + "getTag() {\n";
  out << "  return " + tag_name + ";\n";
  out << "}\n";
  out << "\n";

  // value ctor, getter, setter for each field
  for (const auto& variable : decl->GetFields()) {
    auto var_name = variable->GetName();
    auto var_type = JavaSignatureOf(variable->GetType(), typenames);
    auto raw_type = variable->GetType().ToString();

    out << "// " + raw_type + " " + var_name + "\n";
    // value ctor
    out << variable->GetType().GetComments() + "\n";
    out << "public static " + clazz + " " + var_name + "(" + var_type + " " + value_name + ") {\n";
    out << "  return new " + clazz + "(" + var_name + ", " + value_name + ");\n";
    out << "}\n";
    // getter
    if (variable->GetType().IsGeneric()) {
      out << "@SuppressWarnings(\"unchecked\")\n";
    }
    out << "public " + var_type + " " + getter_name(*variable) + "() {\n";
    out << "  _assertTag(" + var_name + ");\n";
    out << "  return (" + var_type + ") " + value_name + ";\n";
    out << "}\n";
    // setter
    out << "public void " + setter_name(*variable) + "(" + var_type + " " + value_name + ") {\n";
    out << "  _set(" + var_name + ", " + value_name + ");\n";
    out << "}\n";
    out << "\n";
  }

  if (decl->IsVintfStability()) {
    out << "@Override\n";
    out << "public final int getStability() {\n";
    out << "  return android.os.Parcelable.PARCELABLE_STABILITY_VINTF;\n";
    out << "}\n";
    out << "\n";
  }

  out << "public static final android.os.Parcelable.Creator<" << clazz << "> CREATOR = "
      << "new android.os.Parcelable.Creator<" << clazz << ">() {\n";
  out << "  @Override\n";
  out << "  public " << clazz << " createFromParcel(android.os.Parcel _aidl_source) {\n";
  out << "    return new Union(_aidl_source);\n";  // to avoid unnecessary allocation of "default"
  out << "  }\n";
  out << "  @Override\n";
  out << "  public " << clazz << "[] newArray(int _aidl_size) {\n";
  out << "    return new " << clazz << "[_aidl_size];\n";
  out << "  }\n";
  out << "};\n";

  auto write_to_parcel = [&](const AidlTypeSpecifier& type, std::string name, std::string parcel) {
    string code;
    CodeWriterPtr writer = CodeWriter::ForString(&code);
    CodeGeneratorContext context{
        .writer = *(writer.get()),
        .typenames = typenames,
        .type = type,
        .parcel = parcel,
        .var = name,
        .is_return_value = false,
    };
    WriteToParcelFor(context);
    writer->Close();
    return code;
  };

  out << "@Override\n";
  out << "public final void writeToParcel(android.os.Parcel _aidl_parcel, int _aidl_flag) {\n";
  out << "  " + write_to_parcel(tag_type_specifier, tag_name, "_aidl_parcel");
  out << "  switch (" + tag_name + ") {\n";
  for (const auto& variable : decl->GetFields()) {
    out << "  case " + variable->GetName() + ":\n";
    out << "    " +
               write_to_parcel(variable->GetType(), getter_name(*variable) + "()", "_aidl_parcel");
    out << "    break;\n";
  }
  out << "  }\n";
  out << "}\n";

  // keep this across different fields in order to create the classloader
  // at most once.
  bool is_classloader_created = false;
  auto read_from_parcel = [&](const AidlTypeSpecifier& type, std::string name, std::string parcel) {
    string code;
    CodeWriterPtr writer = CodeWriter::ForString(&code);
    CodeGeneratorContext context{
        .writer = *(writer.get()),
        .typenames = typenames,
        .type = type,
        .parcel = parcel,
        .var = name,
        .is_classloader_created = &is_classloader_created,
    };
    CreateFromParcelFor(context);
    writer->Close();
    return code;
  };

  // Not override, but as a user-defined parcelable, this method should be public
  out << "public void readFromParcel(android.os.Parcel _aidl_parcel) {\n";
  out << "  " + tag_type + " _aidl_tag;\n";
  out << "  " + read_from_parcel(tag_type_specifier, "_aidl_tag", "_aidl_parcel");
  out << "  switch (_aidl_tag) {\n";
  for (const auto& variable : decl->GetFields()) {
    auto var_name = variable->GetName();
    auto var_type = JavaSignatureOf(variable->GetType(), typenames);
    out << "  case " + var_name + ": {\n";
    out << "    " + var_type + " _aidl_value;\n";
    out << "    " + read_from_parcel(variable->GetType(), "_aidl_value", "_aidl_parcel");
    out << "    _set(_aidl_tag, _aidl_value);\n";
    out << "    return; }\n";
  }
  out << "  }\n";
  out << "  throw new RuntimeException(\"union: out of range: \" + _aidl_tag);\n";
  out << "}\n";

  out << "@Override\n";
  out << "public int describeContents() {\n";
  out << "  return 0;\n";
  out << "}\n";
  out << "\n";

  // helper: _assertTag
  out << "private void _assertTag(" + tag_type + " tag) {\n";
  out << "  if (getTag() != tag) {\n";
  out << "    throw new IllegalStateException(\"bad access: \" + _tagString(tag) + \", \" + "
         "_tagString(getTag()) + \" is available.\");\n";
  out << "  }\n";
  out << "}\n";
  // helper: _tagString
  out << "private String _tagString(" + tag_type + " " + tag_name + ") {\n";
  out << "  switch (" + tag_name + ") {\n";
  for (const auto& variable : decl->GetFields()) {
    auto var_name = variable->GetName();
    out << "  case " + var_name + ": return \"" + var_name + "\";\n";
  }
  out << "  }\n";
  out << "  throw new IllegalStateException(\"unknown field: \" + " + tag_name + ");\n";
  out << "}\n";
  // helper: _set
  out << "private void _set(" + tag_type + " tag, Object value) {\n";
  out << "  this." + tag_name + " = tag;\n";
  out << "  this." + value_name + " = value;\n";
  out << "}\n";

  out.Dedent();
  out << "}\n";
}

std::string dump_location(const AidlNode& method) {
  return method.PrintLocation();
}

std::string generate_java_unsupportedappusage_parameters(const AidlAnnotation& a) {
  const std::map<std::string, std::string> params = a.AnnotationParams(ConstantValueDecorator);
  std::vector<string> parameters_decl;
  for (const auto& name_and_param : params) {
    const std::string& param_name = name_and_param.first;
    const std::string& param_value = name_and_param.second;
    parameters_decl.push_back(param_name + " = " + param_value);
  }
  parameters_decl.push_back("overrideSourcePosition=\"" + dump_location(a) + "\"");
  return "(" + Join(parameters_decl, ", ") + ")";
}

std::vector<std::string> generate_java_annotations(const AidlAnnotatable& a) {
  std::vector<std::string> result;
  if (a.IsHide()) {
    result.emplace_back("@android.annotation.Hide");
  }

  const AidlAnnotation* unsupported_app_usage = a.UnsupportedAppUsage();
  if (unsupported_app_usage != nullptr) {
    result.emplace_back("@android.compat.annotation.UnsupportedAppUsage" +
                        generate_java_unsupportedappusage_parameters(*unsupported_app_usage));
  }

  auto strip_double_quote = [](const AidlTypeSpecifier& type,
                               const std::string& raw_value) -> std::string {
    if (!StartsWith(raw_value, "\"") || !EndsWith(raw_value, "\"")) {
      AIDL_FATAL(type) << "Java passthrough annotation " << raw_value << " is not properly quoted";
      return "";
    }
    return raw_value.substr(1, raw_value.size() - 2);
  };

  for (const auto& annotation : a.GetAnnotations()) {
    if (annotation.GetType() == AidlAnnotation::Type::JAVA_PASSTHROUGH) {
      for (const auto& name_and_param : annotation.AnnotationParams(strip_double_quote)) {
        if (name_and_param.first == "annotation") {
          result.emplace_back(name_and_param.second);
          break;
        }
      }
    }
  }

  return result;
}

}  // namespace java
}  // namespace aidl
}  // namespace android
