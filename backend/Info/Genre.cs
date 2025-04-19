using System.Diagnostics.CodeAnalysis;

namespace backend.Info;

public class Genre
{
    public Genre() {} 
    public Genre(int id, string name, string? description) 
    {
        this.Id = id;
        this.Name = name;
        this.Description = description;
    }

    public int Id { get; set; }
    public string Name { get; set; } = string.Empty;
    public string? Description { get; set; }

    public List<Book> Books { get; set; } = [];
}