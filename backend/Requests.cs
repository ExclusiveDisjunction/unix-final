

namespace backend;

record UserInformation(string Username, string FirstName, string LastName, string Jwt);

record SignInResponse(bool IsOk, string? Message, UserInformation? Info);

record AddBookRequest
{
    public required string Title { get; init; }
    public required string AuthorID { get; init; }
    public required string GroupName { get; init; }
    public required short Rating { get; init; }
    public required bool IsFavorite { get; init; }
    public required List<String> Genres { get; init; }
}

record AddGenreRequest
{
    public int Id { get; init; }
    public required string Name { get; init; }
    public string? Description { get; init; }
}