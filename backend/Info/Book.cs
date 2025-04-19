using System.Runtime.InteropServices.Swift;

namespace backend.Info;

public class Book
{
    public Book() 
    {
        
    }
    public Book(int id, string title, int author_id, int group_id, List<Genre> genres, short rating, bool is_favorite)
    {
        this.Id = id;
        this.Title = title;
        this.AuthorId = author_id;
        this.GroupId = group_id;
        this.Rating = rating;
        this.Genres = genres;
        this.IsFavorite = is_favorite;
    }
    public Book(int id, string title, Author author, Group group, List<Genre> genres, short rating, bool is_favorite) 
    {
        this.Id = id;
        this.Title = title;
        this.Author = author;
        this.AuthorId = author.Id;
        this.Group = group;
        this.GroupId = group.Id;
        this.Genres = genres;
        this.Rating = rating;
        this.IsFavorite = is_favorite;
    }
    
    public required int Id { get; init; }
    public required string Title { get; set; }
    public required int AuthorId { get; set; }
    public required int GroupId { get; set; }
    public required short Rating { get; set; }
    public required bool IsFavorite { get; set; }
    
    public List<Genre> Genres { get; set; } = [];
    public Author? Author { get; set; }
    public Group? Group { get; set; }
}