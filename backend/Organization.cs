using System.Text.Json;
using Microsoft.EntityFrameworkCore;

namespace backend;

using Info;

public class Organization
{
    internal static async Task AddGenre(HttpContext context) 
    {
        if (Program.App is null) 
        {
            await Console.Error.WriteLineAsync("Application is not created.");
            context.Response.StatusCode = 500;
            return;
        }

        try
        {
            var info = await context.Request.ReadFromJsonAsync<AddGenreRequest>();
            if (info is null)
            {
                Program.App.Logger.LogError(
                    "The provided message is not valid JSON, and could not be represented as a AddGenreRequest.");
                context.Response.StatusCode = 400;
                return;
            }

            using var scope = Program.App.Services.CreateScope();
            var database = scope.ServiceProvider.GetRequiredService<Database>();

            Program.App.Logger.LogInformation($"Adding genre with name '{info.Name}'");
            var genre = new Genre(info.Name, info.Description);
            await database.Genres.AddAsync(genre);
            await database.SaveChangesAsync();

            context.Response.StatusCode = 200;
        }
        catch (JsonException jsonEx)
        {
            Program.App.Logger.LogError($"Unable to parse request '{jsonEx}'");
        }
        catch (InvalidOperationException ex)
        {
            Program.App.Logger.LogWarning($"The database could not be retrieved, error '{ex}");
        }
        catch (DbUpdateException)
        {
            Program.App.Logger.LogWarning("A genre with this name already exists.");
            context.Response.StatusCode = 409;
        }
    }
    
    internal static async Task AddGroup(HttpContext context, string username)
    {
        if (Program.App is null)
        {
            await Console.Error.WriteLineAsync("Application is not created.");
            return;
        }

        if (username.Length == 0)
        {
            context.Response.StatusCode = 400;
            return;
        }

        Program.App.Logger.LogInformation($"Adding group request to user '{username}'");
        Database database;
        using var scope = Program.App.Services.CreateScope();
        try
        {
            database = scope.ServiceProvider.GetRequiredService<Database>(); 
        }
        catch (InvalidOperationException ex)
        {
            Program.App.Logger.LogError($"Unable to get the database {ex}");
            context.Response.StatusCode = 400;
            return;
        }

        AddGroupRequest? request;
        try
        {
            request = await context.Request.ReadFromJsonAsync<AddGroupRequest>();
        }
        catch
        {
            request = null;
        }

        if (request is null)
        {
            Program.App.Logger.LogWarning("Unable to parse JSON content into a AddGroupRequest");
            context.Response.StatusCode = 400;
            return;
        }

        Program.App.Logger.LogDebug("Looking up user");
        var user = await UserMgnt.GetUserAsync(database, username);
        if (user is null)
        {
            Program.App.Logger.LogWarning("The user could not be found.");
            context.Response.StatusCode = 404;
            return;
        }
        
        Program.App.Logger.LogInformation($"Inserting new group '{request.Name} under user {user.Username}");
        try
        {
            var newGroup = new Group(user.Username, request.Name, request.Description);
            await database.Groups.AddAsync(newGroup);
            await database.SaveChangesAsync();
        }
        catch (DbUpdateException)
        {
            Program.App.Logger.LogWarning($"The specific group already exists under this user. The request is not made.");
            context.Response.StatusCode = 409;
            return;
        }

        context.Response.StatusCode = 200;
    }
    
    internal static async Task GetGenres(HttpContext context)
    {
        if (Program.App is null)
        {
            await Console.Error.WriteLineAsync("Application is not created.");
            return;
        }
        
        
        
        using var scope = Program.App.Services.CreateScope();
        Database database;
        try
        {
            database = scope.ServiceProvider.GetRequiredService<Database>(); 
        }
        catch (InvalidOperationException ex)
        {
            Program.App.Logger.LogError($"Unable to get the database {ex}");
            context.Response.StatusCode = 400;
            return;
        }
        
        var result = await database.Genres.ToArrayAsync();
        context.Response.StatusCode = 200;
        await context.Response.WriteAsJsonAsync(result);
    }
    
    internal static async Task GetGroups(HttpContext context, string username)
    {
        if (Program.App is null)
        {
            await Console.Error.WriteLineAsync("Application is not created.");
            return;
        }

        using var scope = Program.App.Services.CreateScope();
        Database? database;
        try
        {
            database = scope.ServiceProvider.GetService<Database>();
        }
        catch (InvalidOperationException ex)
        {
            Program.App.Logger.LogError($"Invalid operation error '{ex}'");
            database = null;
        }

        if (database is null)
        {
            Program.App.Logger.LogError("The database could not be loaded.");
            context.Response.StatusCode = 500;
            return;
        }
        
        var targetUser = await UserMgnt.GetUserAsync(database, username);
        if (targetUser is null)
        {
            Program.App.Logger.LogWarning("The specified user could not be found.");
            context.Response.StatusCode = 404;
            return;
        }

        if (!UserMgnt.ValidateJwtAgainstUser(targetUser.Username, context))
        {
            Program.App.Logger.LogWarning("The username provided does not match their token.");
            context.Response.StatusCode = 403;
            return;
        }
        
        Program.App.Logger.LogInformation($"Getting groups for {username}");
        
        context.Response.StatusCode = 200;
        await context.Response.WriteAsJsonAsync(targetUser.Groups);
    }
}