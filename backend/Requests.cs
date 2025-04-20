

namespace backend;

record UserInformation(string Username, string FirstName, string LastName, string Jwt);

record SignInRequest(string Username, string Password);

record CreateUserRequest(string Username, string FirstName, string LastName, string Password);

record AddBookRequest(
    string JWT,
    string Title,
    int Author,
    string GroupName,
    short Rating,
    bool IsFavorite,
    List<String> Genres
);

record AddGroupRequest(string Jwt, string Name, string? Description);

record AddGenreRequest(string Jwt, string Name, string? Description);