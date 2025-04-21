namespace backend.Info;

public class User : IDbUpdatable<EditUserRequest>
{
    public string Username { get; set; } = string.Empty;
    public string FirstName { get; set; } = string.Empty;
    public string LastName { get; set; } = string.Empty;
    public string PasswordHash { get; set; } = string.Empty;

    public List<Group> Groups { get; set; } = [];

    public void UpdateFrom(EditUserRequest source, Database database)
    {
        if (source.FirstName is not null)
            FirstName = source.FirstName;

        if (source.LastName is not null)
            LastName = source.LastName;

        if (source.Password is not null)
            PasswordHash = BCrypt.Net.BCrypt.HashPassword(source.Password);
    }

    public UserInformation GenerateData(string jwt)
    {
        return new UserInformation(Username, FirstName, LastName, jwt);
    }
}

// Response
public record UserInformation(string Username, string FirstName, string LastName, string Jwt);

// Request
public record SignInRequest(string Username, string Password);
public record CreateAccountRequest(string Username, string FirstName, string LastName, string Password);
public record EditUserRequest(bool Keep, string? FirstName, string? LastName, string? Password) : IUpdatableRecord;
