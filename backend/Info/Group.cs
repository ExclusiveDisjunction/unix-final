using Microsoft.AspNetCore.Components.Forms;

namespace backend.Info;

public class Group : IDbModify<Group, OrganizationData, EditOrganizationRequest, OrganizationData>
{
    public int Id { get; set; }
    public string ParentId { get; set; } = string.Empty;
    public string Name { get; set; } = string.Empty;
    public string? Description { get; set; }

    
    public User? Parent { get; set; }
    public List<Book> Books { get; set; } = [];
    
    public static Task<Group> CreateFromAsync(OrganizationData source, Database database)
    {
        return new Task<Group>(() => new Group
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