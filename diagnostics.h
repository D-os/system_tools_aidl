/*
 * Copyright (C) 2020, The Android Open Source Project
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
#pragma once

#include <map>
#include <stack>
#include <string>
#include <vector>

#include <android-base/scopeguard.h>
#include <android-base/strings.h>

class AidlLocation;
class AidlErrorLog;

namespace android {
namespace aidl {

enum class DiagnosticSeverity {
  DISABLED,
  WARNING,
  ERROR,
};

enum class DiagnosticID {
#define DIAG(ENUM, NAME, ENABLED) ENUM,
#include "diagnostics.inc"
#undef DIAG
};

class DiagnosticMapping {
 public:
  DiagnosticSeverity Severity(DiagnosticID id) const;
  void Severity(DiagnosticID id, DiagnosticSeverity severity);

 private:
  std::map<DiagnosticID, DiagnosticSeverity> mapping_;
};

class DiagnosticsContext {
 public:
  DiagnosticsContext(DiagnosticMapping mapping) : mapping_(mapping) {}
  virtual ~DiagnosticsContext() = default;
  AidlErrorLog Report(const AidlLocation& loc, DiagnosticID id);

 protected:
  virtual AidlErrorLog DoReport(const AidlLocation& loc, DiagnosticID id,
                                DiagnosticSeverity severity) = 0;

 private:
  DiagnosticMapping mapping_;
};

struct DiagnosticOption {
  DiagnosticID id;
  const std::string name;
  bool default_enabled;
};

extern const std::map<std::string, DiagnosticOption> kAllDiagnostics;

// relying on Argument-dependent lookup
std::string to_string(DiagnosticID id);

}  // namespace aidl
}  // namespace android