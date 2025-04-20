namespace backend.Info;

public class Author
{
    public Author() { } 
    public Author(int id, string first_name, string last_name, List<Book> books) 
    {
        this.Id = id;
        this.FirstName = first_name;
        this.LastName = last_name;
        this.Books = books;
    }

    public int Id { get; set; }
    public required string FirstName { get; set; }
    public required string LastName { get; set; }

    public List<Book> Books { get; set; } = [];
}