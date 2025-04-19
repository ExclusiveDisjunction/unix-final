namespace backend.Info;

public class Group
{
    public Group() 
    {

    }
    public Group(int id, string parent_id, string name, string? description) 
    {
        this.Id = id;
        this.ParentId = parent_id;
        this.Name = name;
        this.Description = description;
    }

    public int Id { get; set; }
    public string ParentId { get; set; } = string.Empty;
    public string Name { get; set; } = string.Empty;
    public string? Description { get; set; }
    
    public User? Parent { get; set; }
    public List<Book> Books { get; set; } = [];
}