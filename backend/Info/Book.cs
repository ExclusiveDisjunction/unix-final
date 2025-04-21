using Microsoft.EntityFrameworkCore;

namespace backend.Info;

public class Book : IDbModify<Book, AddBookRequest, EditBookRequest, BookData>
{
    public int Id { get; set; }
    public string Title { get; set; } = string.Empty;
    public int AuthorId { get; set; }
    public int GroupId { get; set; }
    public short Rating { get; set; }
    public bool IsFavorite { get; set; }
    
    public List<Genre> Genres { get; set; } = [];
    public Author? Author { get; set; }
    public Group? Group { get; set; }
    
    public static Book CreateFrom(AddBookRequest source, Database database)
    {
        //May throw
        var group = database.Groups.Single(u => u.Name == source.GroupName);
        var author = database.Authors.Single(u => u.Id == source.Author);
        var genresQuery = from genre in database.Genres
            where source.Genres.Contains(genre.Name)
            select genre;
        var genres = genresQuery.ToList();

        return new Book
        {
            Title = source.Title,
            Rating = source.Rating,
            IsFavorite = source.IsFavorite,
            GroupId = group.Id,
            Group = group,
            AuthorId = author.Id,
            Author = author,
            Genres = genres
        };
    }

    public void UpdateFrom(EditBookRequest source, Database database)
    {
        if (source.Title is not null)
        {
            Title = source.Title;
        }

        if (source.Group is not null)
        {
            var targetGroup = source.Group;
            var newGroup = database.Groups.Single(g => g.Name == targetGroup);
            GroupId = newGroup.Id;
            Group = newGroup;
        }

        if (source.Rating is not null)
        {
            Rating = (short)source.Rating;
        }

        if (source.IsFavorite is not null)
        {
            IsFavorite = (bool)source.IsFavorite;
        }

        if (source.Genres is not null)
        {
            var asSet = new HashSet<string>(source.Genres);
            var query = from genre in database.Genres
                where asSet.Contains(genre.Name)
                select genre;

            var newGenres = query.ToList();
            Genres = newGenres;
        }
    }

    public BookData GenerateData()
    {
        return new BookData(
            Title,
            AuthorId,
            Rating,
            IsFavorite,
            Genres.Select(u => u.GenerateData()).ToList()
        );
    }
}

public record AddBookRequest(
    string Title,
    int Author,
    string GroupName,
    short Rating,
    bool IsFavorite,
    List<string> Genres
);

public record BookData(
    string Title,
    int Author,
    short Rating,
    bool IsFavorite,
    List<OrganizationData> Genres
);

public record EditBookRequest(
    string OldTitle,
    bool Keep,
    string? Group,
    string? Title,
    short? Rating,
    bool? IsFavorite,
    List<string>? Genres
) : IUpdatableRecord;