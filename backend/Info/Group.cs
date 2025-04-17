namespace backend.Info;

public class Group
{
    public int Id { get; set; }
    public string ParentId { get; set; }
    public string Name { get; set; }
    public string? Description { get; set; }
    
    public User? Parent { get; set; }
    public List<Book> Books { get; set; } = new();
}