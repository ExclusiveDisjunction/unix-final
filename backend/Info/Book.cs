using System.Runtime.InteropServices.Swift;

namespace backend.Info;

public class Book
{
    public Book(int id, string title)
    {
        this.Id = id;
        this.Title = title;
    }
    
    public int Id { get; init; }
    public string Title { get; set; }
    public int AuthorId { get; set; }
    public int GroupId { get; set; }
    public short Rating { get; set; }
    public bool IsFavorite { get; set; }
    
    public List<Genre> Genres { get; set; } = new();
    public Author? Author { get; set; }
    public Group? Group { get; set; }
}