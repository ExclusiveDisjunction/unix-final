using System.Text.Json;
using Microsoft.EntityFrameworkCore;

namespace backend;

using Info;

public static class OrgManagement
{
    internal static async Task AddGenre(HttpContext context)
    {
        var app = AppTools.GetApp();

        var request = await AppTools.ParseRequestJson<OrganizationData>(app.Logger, context);
        if (request is null)
            return;
        
        using var scope = app.Services.CreateScope();
        var database = AppTools.GetDatabase(app.Logger, context, scope);
        if (database is null)
            return;
        
        app.Logger.LogInformation($"Adding genre with name '{request.Name}'");
        var genre = Genre.CreateFrom(request, database);
        await database.Genres.AddAsync(genre);
        app.Logger.LogInformation($"Genre added. Saving.");
        await database.SaveChangesAsync();
        app.Logger.LogInformation($"Genre added. Saving complete.");

        context.Response.StatusCode = 200;
    }
    internal static async Task GetGenres(HttpContext context)
    {
        var app = AppTools.GetApp();
        
        using var scope = app.Services.CreateScope();
        var database = AppTools.GetDatabase(app.Logger, context, scope);
        if (database is null)
            return;
        
        var result = await database.Genres.ToArrayAsync();
        context.Response.StatusCode = 200;
        await context.Response.WriteAsJsonAsync(result);
    }

    internal static async Task EditGenre(HttpContext context)
    {
        var app = AppTools.GetApp();
        
        using var scope = app.Services.CreateScope();
        var database = AppTools.GetDatabase(app.Logger, context, scope);
        if (database is null)
            return;
        
        var request = await AppTools.ParseRequestJson<EditOrganizationRequest>(app.Logger, context);
        if (request is null)
            return;

        var target = await database.Genres.SingleOrDefaultAsync(u => u.Name == request.OldName);
        if (target is null)
        {
            app.Logger.LogWarning($"The genre with name '{request.OldName}' was not found.");
            context.Response.StatusCode = 404;
            return;
        }

        await AppTools.UpdateInstance(target, request, app.Logger, database, context);
    }
    
    internal static async Task AddGroup(HttpContext context, string username)
    {
        var app = AppTools.GetApp();
        
        var request = await AppTools.ParseRequestJson<OrganizationData>(app.Logger, context);
        if (request is null)
            return;
        
        using var scope = app.Services.CreateScope();
        var database = AppTools.GetDatabase(app.Logger, context, scope);
        if (database is null)
            return;

        var user = await UserManagement.GetUserAsync(database, username);
        if (user is null)
            return;

        if (!UserManagement.ValidateJwtAgainstUser(username, context))
            return;
        
        app.Logger.LogInformation($"Inserting new group '{request.Name} under user {user.Username}");
        
        try
        {
            var newGroup = Group.CreateFrom(request, database);
            newGroup.ParentId = username;
            newGroup.Parent = user;
            
            await database.Groups.AddAsync(newGroup);
            await database.SaveChangesAsync();
            
            context.Response.StatusCode = 200;
        }
        catch (DbUpdateException)
        {
            app.Logger.LogWarning("The specific group already exists under this user. The request is not made.");
            context.Response.StatusCode = 409;
        }
    }
    internal static async Task GetGroups(HttpContext context, string username)
    {
        var app = AppTools.GetApp();
        
        using var scope = app.Services.CreateScope();
        var database = AppTools.GetDatabase(app.Logger, context, scope);
        if (database is null)
            return;
        
        var user = await UserManagement.GetUserAsync(database, username);
        if (user is null)
            return;

        if (!UserManagement.ValidateJwtAgainstUser(username, context))
            return;
                
        app.Logger.LogInformation($"Getting groups for {username}");
        
        context.Response.StatusCode = 200;
        var data = user.Groups.Select(u => u.GenerateData()).ToArray();
        await context.Response.WriteAsJsonAsync(data);
    }

    internal static async Task EditGroup(HttpContext context, string username)
    {
        var app = AppTools.GetApp();
        
        using var scope = app.Services.CreateScope();
        var database = AppTools.GetDatabase(app.Logger, context, scope);
        if (database is null)
            return;
        
        var request = await AppTools.ParseRequestJson<EditOrganizationRequest>(app.Logger, context);
        if (request is null)
            return;
        
        if (!AppTools.EnforceJwtUserValidation(app.Logger, context, username))
            return;

        var target = await database.Groups.SingleOrDefaultAsync(u => u.Name == request.OldName && u.ParentId == username);
        if (target is null)
        {
            app.Logger.LogWarning($"The group with name '{request.OldName}' was not found.");
            context.Response.StatusCode = 404;
            return;
        }

        await AppTools.UpdateInstance(target, request, app.Logger, database, context);
    }
}