namespace backend.Info;

public class Genre
{
    public int Id { get; set; }
    public string Name { get; set; }
    public string Description { get; set; }

    public List<Book> Books { get; set; } = new();
}