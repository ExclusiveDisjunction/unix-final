using System.Runtime.InteropServices.Swift;

namespace backend.Info;

public class Book
{
    public Book() 
    {
        
    }
    public Book(int id, string title, int authorId, int groupId, List<Genre> genres, short rating, bool is_favorite)
    {
        Id = id;
        Title = title;
        AuthorId = authorId;
        GroupId = groupId;
        Rating = rating;
        Genres = genres;
        IsFavorite = is_favorite;
    }
    public Book(int id, string title, Author author, Group group, List<Genre> genres, short rating, bool is_favorite) 
    {
        Id = id;
        Title = title;
        Author = author;
        AuthorId = author.Id;
        Group = group;
        GroupId = group.Id;
        Genres = genres;
        Rating = rating;
        IsFavorite = is_favorite;
    }
    
    public int Id { get; init; }
    public string Title { get; set; } = string.Empty;
    public int AuthorId { get; set; }
    public int GroupId { get; set; }
    public short Rating { get; set; }
    public bool IsFavorite { get; set; }
    
    public List<Genre> Genres { get; set; } = [];
    public Author? Author { get; set; }
    public Group? Group { get; set; }
}