namespace backend.Info;

public class Author
{
    public Author() { } 
    public Author(int id, string firstName, string lastName, List<Book> books) 
    {
        Id = id;
        FirstName = firstName;
        LastName = lastName;
        Books = books;
    }

    public int Id { get; set; }
    public required string FirstName { get; set; } = String.Empty;
    public required string LastName { get; set; } = String.Empty;

    public List<Book> Books { get; set; } = [];
}