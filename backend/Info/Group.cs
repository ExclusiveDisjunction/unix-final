namespace backend.Info;

public class Group
{
    public Group() 
    {

    }
    public Group(int id, string parentId, string name, string? description) 
    {
        Id = id;
        ParentId = parentId;
        Name = name;
        Description = description;
    }
    public Group(string parentId, string name, string? description)
    {
        ParentId = parentId;
        Name = name;
        Description = description;
    }

    public int Id { get; set; }
    public string ParentId { get; set; } = string.Empty;
    public string Name { get; set; } = string.Empty;
    public string? Description { get; set; }
    
    public User? Parent { get; set; }
    public List<Book> Books { get; set; } = [];
}