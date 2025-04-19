

namespace backend;

record AddBookRequest
{
    public int Id { get; init; }
    public required string Title { get; init; }
}

record AddGenreRequest
{
    public int Id { get; set; }
    public required string Name { get; set; }
    public string? Description { get; set; }
}