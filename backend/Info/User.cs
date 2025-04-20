namespace backend.Info;

public class User
{
    public User() { }
    public User(string username, string firstName, string lastName) 
    {
        Username = username;
        FirstName = firstName;
        LastName = lastName;
    }

    public string Username { get; set; } = string.Empty;
    public string FirstName { get; set; } = string.Empty;
    public string LastName { get; set; } = string.Empty;

    public List<Group> Groups { get; set; } = [];
}