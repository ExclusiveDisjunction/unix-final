namespace backend.Info;

public record OrganizationData(string Name, string? Description);
//Request
public record EditOrganizationRequest(string OldName, bool Keep, string? Name, string? Description) : IUpdatableRecord;


public class Genre : IDbModify<Genre, OrganizationData, EditOrganizationRequest, OrganizationData>
{
    public string Name { get; set; } = string.Empty;
    public string? Description { get; set; }

    public List<Book> Books { get; set; } = [];

    public static Task<Genre> CreateFromAsync(OrganizationData source, Database database)
    {
        return new Task<Genre>(() => new Genre
        {
            Name = source.Name,
            Description = source.Description
        });
    }

    public Task UpdateFromAsync(EditOrganizationRequest source, Database database)
    {
        return new Task(() =>
        {
            if (source.Name is not null)
                Name = source.Name;

            if (source.Description is not null)
                Description = source.Description;
        });
    }

    public OrganizationData GenerateData()
    {
        return new OrganizationData(Name, Description);
    }
}