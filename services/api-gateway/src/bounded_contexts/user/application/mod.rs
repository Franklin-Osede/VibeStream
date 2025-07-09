pub mod dtos;
pub mod commands;
pub mod queries;
pub mod handlers;
pub mod services;
pub mod events;

// Re-export main types
pub use dtos::{
    CreateUserDto, UpdateUserDto, UserResponseDto, UserListResponseDto,
    UserProfileDto, UserStatsDto, UserPreferencesDto, LoginDto,
    ChangePasswordDto, LinkWalletDto, UpdateProfileDto
};

pub use commands::{
    RegisterUser, RegisterUserResult, UpdateUser, FollowUser, DeleteUser,
    RegisterUserHandler, UpdateUserHandler, FollowUserHandler
};

pub use queries::{
    GetUserQuery, GetUserByEmailQuery, GetUserByUsernameQuery,
    GetUserStatsQuery, GetUserPreferencesQuery, SearchUsersQuery,
    GetUserListQuery, GetTopUsersQuery
};

pub use handlers::{
    UserCommandHandler, UserQueryHandler
};

pub use services::{
    UserApplicationService
}; 