/// @title Core module for the application
/// @notice This module provides access to formatting, performance, and validation functionality
/// @dev Contains submodules for different aspects of the application logic

/// @notice Formatting utilities and functions
pub mod formatting;

/// @notice Performance monitoring and optimization tools
pub mod performance;

/// @notice Input validation and data verification utilities
pub mod validation;

/// @notice Re-export of ValidationEngine for convenient access
/// @dev This allows users to import ValidationEngine directly from this module
pub use validation::ValidationEngine;
