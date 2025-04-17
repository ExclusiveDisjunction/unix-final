namespace backend.Info;

public class Book
{
    public int Id { get; set; }
    public string Title { get; set; }
    public int AuthorId { get; set; }
    public int GroupId { get; set; }
    public short Rating { get; set; }
    public bool IsFavorite { get; set; }
    
    public List<Genre> Genres { get; set; } = new();
    public Author? Author { get; set; }
    public Group? Group { get; set; }
}