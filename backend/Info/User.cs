namespace backend.Info;

public class User
{
    public User() { }
    public User(string username, string first_name, string last_name) 
    {
        this.Username = username;
        this.FirstName = first_name;
        this.LastName = last_name;
    }

    public string Username { get; set; } = string.Empty;
    public string FirstName { get; set; } = string.Empty;
    public string LastName { get; set; } = string.Empty;

    public List<Group> Groups { get; set; } = new();
}