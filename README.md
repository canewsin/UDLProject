# Unified Design Language (UDL) Project

## Abstract:

The Unified Design Language (UDL) project aims to create a standardized design language that can be used by developers, designers, and product managers. UDL defines design tokens, data sources and interfaces for stateful designs via standardized structures and conventions.

## Table of Contents
- [Introduction](#introduction)
- [Current State of Project](#current-state-of-project)
- [Expected Final State](#expected-final-state)
- [Language Support](#language-support)
- [TODO](#todo)
- [Basic Example](#basic-example)
- [More Complete Example](#more-complete-example)

## Introduction:
Stateful designs are complex and require a standardized approach to ensure consistency across different UI frameworks and OS Native UI engines. UDL provides a set of design tokens, data sources and interfaces that can be used to create stateful designs. These design tokens are defined in a standardized format that can be used by developers, designers, and product managers.

Design Tokens here refers to nomenclature of components, colors, typography, spacing, models and data contracts. These tokens can be used to generate code for UI components, stylesheets, and other design artifacts via udl-gen without any additional rewriting code manually including dynamic(Stateful) components.

UDL defines only naming conventions. Presentation layer is left to Design Systems like Material Design, Bootstrap, Tailwind CSS, etc.

## Current State of Project:
In the process of defining and implementing class and enum definitions of udl data.

## Expected Final State:
### How Usual Product Development Works
In a product development environment, product managers create product documents, goals, references and examples of final products. UI designers gives a shape to the product, developers then implement the design via code. Their implementations looped back to product managers for feedback and iteration. At each stage, each has their own non standardized way with manual implementations that consumes unnecessary time and resources. Let's say UI designer designs a Page in Figma, developers implement the design via code, and product managers provide feedback and iterate on the design. This process continues until the product is complete. Developers redo same process UI designers already did in the previous stage, this repeats until the product is complete. Sometimes this process becomes complicated when dealing with different device platforms and UI frameworks.

### What UDL Aimed to Solve
With UDL, UI Designers/Developers define a structure of tokens that can be used to generate code for UI components, stylesheets, and other design artifacts via udl-gen without rewriting code manually including dynamic(Stateful) components.

## Language Support:
- [x] Dart/Flutter
- [ ] WIP: Rust

## TODO:
 - [x] WIP: Implement data class and enums
 - [ ] Implement interfaces for stateful designs
 - [ ] Implement Mock Implementation for design token data
 - [ ] Define design tokens names for components, colors, typography, spacing, models and data contracts.
 
## Basic Example:
```yaml
models:
  - id: ApiError
    description: "Standard error response"
    properties:
      code: string
      message: string?
      timestamp: datetime
```

### Generated Dart Code:
```dart
/// Standard error response
class ApiError {
  final String code;
  final String? message;
  final DateTime timestamp;

  const ApiError({
    required this.code,
    this.message,
    required this.timestamp,
  });
}
```

### Generated Rust Code:
```rust
/// Standard error response
pub struct ApiError {
    pub code: String,
    pub message: Option<String>,
    pub timestamp: DateTime,
}
```

## More Complete Example:
```yaml
udl_version: 0.0.1

project:
  name: BillnChill App
  version: 0.0.1
  description: "Simplified Billing for Business"
  namespace: "com.billnchill.app"
  models_only: true
  target_platforms:
    - flutter
  authors:
    - name: "Pramukesh"
      email: "foss@pramukesh.com"
  license: MIT

enums:
  - id: LoginError
    type: "constructor_error"
    variants:
      - id: K_INVALID_EMAIL
        value: "Invalid email address"
        description: "Invalid email address"
        target: "format:email"
      - id: K_INVALID_PASSWORD_MIN
        value: "Invalid password minimum length"
        target: "limit:min"
        description: "Password is too short"
      - id: K_INVALID_PASSWORD_MAX
        value: "Invalid password maximum length"
        target: "limit:max"
        description: "Password is too long"

  - id: UserNameError
    type: "constructor_error"
    variants:
      - id: K_INVALID_USER_NAME_MIN
        value: "Invalid username minimum length"
        target: "limit:min"
        description: "Username is too short"
      - id: K_INVALID_USER_NAME_MAX
        value: "Invalid username maximum length"
        target: "limit:max"
        description: "Username is too long"

models:
  - id: LoginRequest
    description: "User login request"
    error: LoginError
    properties:
      email:
        type: string
        format: email
        description: "User email address"
      password:
        type: string
        limit: 8...32
      remember_me: bool

  - id: User
    error: UserNameError
    description: "User profile data"
    properties:
      id:
        type: string
        format: uuid
      email:
        type: string
        format: email
      name:
        type: string
        limit: 6...100
      phone:
        type: string?
        format: phone
      company: string?
      created_at: datetime
      updated_at: datetime^
      login_status: $enum::LoginStatus
```

### Generated code via udl-gen(Dart)

```dart
import 'package:result_dart/result_dart.dart';

enum LoginError {
  /// Invalid email address
  kInvalidEmail("Invalid email address"),

  /// Password is too short
  kInvalidPasswordMin("Invalid password minimum length"),

  /// Password is too long
  kInvalidPasswordMax("Invalid password maximum length");

  final String value;

  const LoginError(this.value);
}

enum UserNameError {
  /// Username is too short
  kInvalidUserNameMin("Invalid username minimum length"),

  /// Username is too long
  kInvalidUserNameMax("Invalid username maximum length");

  final String value;

  const UserNameError(this.value);
}


/// User login request
class LoginRequest {
  /// User email address
  final String email;
  final String password;
  final bool rememberMe;

  const LoginRequest._({
    required this.email,
    required this.password,
    required this.rememberMe,
  });

  static ResultDart<LoginRequest, LoginError> build({
    required String password,
    required bool rememberMe,
    required String email,
  }) {
    // Format Validator found for email
    // Limit Validator found for password
    if (password.length < 8) {
      return Failure(LoginError.kInvalidPasswordMin);
    }
    if (password.length > 32) {
      return Failure(LoginError.kInvalidPasswordMax);
    }
    return Success(
      LoginRequest._(email: email, password: password, rememberMe: rememberMe),
    );
  }
}

/// User profile data
class User {
  final String? company;
  final DateTime createdAt;
  final String id;
  final String name;
  final LoginStatus loginStatus;
  final DateTime updatedAt;
  final String? phone;
  final String email;

  const User._({
    required this.loginStatus,
    required this.phone,
    required this.name,
    required this.email,
    required this.createdAt,
    required this.company,
    required this.updatedAt,
    required this.id,
  });

  static ResultDart<User, UserNameError> build({
    required String name,
    required String id,
    required DateTime updatedAt,
    required String email,
    required String? phone,
    required String? company,
    required LoginStatus loginStatus,
    required DateTime createdAt,
  }) {
    // Format Validator found for id
    // Limit Validator found for name
    if (name.length < 6) {
      return Failure(UserNameError.kInvalidUserNameMin);
    }
    if (name.length > 100) {
      return Failure(UserNameError.kInvalidUserNameMax);
    }
    // Format Validator found for phone
    // Format Validator found for email
    return Success(
      User._(
        company: company,
        createdAt: createdAt,
        id: id,
        name: name,
        loginStatus: loginStatus,
        updatedAt: updatedAt,
        phone: phone,
        email: email,
      ),
    );
  }
}
```
