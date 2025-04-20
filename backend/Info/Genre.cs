namespace backend.Info;

public class Genre
{
    public Genre() {} 
    public Genre(string name, string? description) 
    {
        Name = name;
        Description = description;
    }
    
    public string Name { get; set; } = string.Empty;
    public string? Description { get; set; }

    public List<Book> Books { get; set; } = [];
}