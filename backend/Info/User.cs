namespace backend.Info;

public class User
{
    public string Username { get; set; }
    public string FirstName { get; set; }
    public string LastName { get; set; }

    public List<Group> Groups { get; set; } = new();
}