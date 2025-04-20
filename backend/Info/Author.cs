namespace backend.Info;

public class Author : IDbModify<Author, AddAuthorRequest, EditAuthorRequest, AuthorData>
{
    public int Id { get; set; }
    public string FirstName { get; set; } = String.Empty;
    public string LastName { get; set; } = String.Empty;
    
    public List<Book> Books { get; set; } = [];

    public static Task<Author> CreateFromAsync(AddAuthorRequest source, Database database)
    {
        return new Task<Author>(() => new Author
        {
            FirstName = source.FirstName,
            LastName = source.LastName
        });
    }

    public Task UpdateFromAsync(EditAuthorRequest source, Database database)
    {
        return new Task(() =>
        {
            if (source.FirstName is not null)
            {
                FirstName = source.FirstName;
            }

            if (source.LastName is not null)
            {
                LastName = source.LastName;
            }
        });
    }

    public AuthorData GenerateData()
    {
        return new AuthorData(FirstName, LastName, Id);
    }
}

public record AddAuthorRequest(string FirstName, string LastName);
public record AuthorData(string FirstName, string LastName, int Id);
public record EditAuthorRequest(int Id, bool Keep, string? FirstName, string? LastName) : IUpdatableRecord;
