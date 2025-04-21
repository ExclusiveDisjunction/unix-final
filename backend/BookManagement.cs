using Microsoft.EntityFrameworkCore;
using backend.Info;

namespace backend;

public static class BookManagement
{
    internal static async Task AddBook(HttpContext context, string username)
    {
        var app = AppTools.GetApp();

        app.Logger.LogInformation("Adding book request");
        var request = await AppTools.ParseRequestJson<AddBookRequest>(app.Logger, context);
        if (request is null)
            return;
        
        using var scope = app.Services.CreateScope();
        var database = AppTools.GetDatabase(app.Logger, context, scope);
        if (database is null)
            return;

        if (!UserManagement.ValidateJwtAgainstUser(username, context))
            return;
        
        try
        {
            var target = Book.CreateFrom(request, database);
            database.Books.Add(target);
            
            await database.SaveChangesAsync();
            context.Response.StatusCode = 200;
        }
        catch (Exception ex)
        {
            app.Logger.LogError($"Unable to insert target '{ex.Message}");
            context.Response.StatusCode = 404;
        }
    }
    internal static async Task GetBooks(HttpContext context, string username)
    {
        var app = AppTools.GetApp();

        app.Logger.LogInformation($"Getting books for {username}");
        
        using var scope = app.Services.CreateScope();
        var database = AppTools.GetDatabase(app.Logger, context, scope);
        if (database is null)
            return;
        
        var selectedUser = await UserManagement.GetUserAsync(database, username);
        if (selectedUser is null)
        {
            app.Logger.LogWarning("The user could not be found.");
            context.Response.StatusCode = 404;
            return;
        }

        if (!UserManagement.ValidateJwtAgainstUser(username, context))
        {
            app.Logger.LogWarning("The user was found, but it did not match their JWT.");
            context.Response.StatusCode = 403;
            return;
        }
        
        var groups = selectedUser.Groups;
        var result = new Dictionary<OrganizationData, List<BookData>>();
        foreach (var group in groups)
        {
            var key = group.GenerateData();
            result[key] = group.Books.Select(u => u.GenerateData()).ToList();
        }

        await context.Response.WriteAsJsonAsync(result);
        context.Response.StatusCode = 200;
    }
    internal static async Task EditBook(HttpContext context, string username)
    {
        var app = AppTools.GetApp();

        app.Logger.LogInformation($"Editing book for {username}");
        
        using var scope = app.Services.CreateScope();
        var database = AppTools.GetDatabase(app.Logger, context, scope);
        if (database is null)
            return;
        
        var request = await AppTools.ParseRequestJson<EditBookRequest>(app.Logger, context);
        if (request is null)
            return;

        if (!AppTools.EnforceJwtUserValidation(app.Logger, context, username))
            return;
        
        var target = await database.Books.SingleOrDefaultAsync(b => b.Title == request.OldTitle);
        if (target is null) 
        {
            app.Logger.LogWarning($"The book with title '{request.OldTitle}' was not found.");
            context.Response.StatusCode = 404;
            return;
        }

        await AppTools.UpdateInstance(target, request, app.Logger, database, context);
    }

    internal static async Task AddAuthor(HttpContext context)
    {
        var app = AppTools.GetApp();
        app.Logger.LogInformation("Adding author");
        
        var request = await AppTools.ParseRequestJson<AddAuthorRequest>(app.Logger, context);
        if (request is null)
            return;
        
        using var scope = app.Services.CreateScope();
        var database = AppTools.GetDatabase(app.Logger, context, scope);
        if (database is null)
            return;
        
        app.Logger.LogInformation("Inserting new author.");

        try
        {
            var newAuthor = Author.CreateFrom(request, database);

            await database.Authors.AddAsync(newAuthor);
            await database.SaveChangesAsync();

            context.Response.StatusCode = 200;
        }
        catch (DbUpdateException)
        {
            app.Logger.LogWarning("The specific author being added currently exists.");
            context.Response.StatusCode = 409;
        }
    }
    internal static async Task GetAuthors(HttpContext context)
    {
        var app = AppTools.GetApp();

        app.Logger.LogInformation("Getting authors");
        using var scope = app.Services.CreateScope();
        var database = AppTools.GetDatabase(app.Logger, context, scope);
        if (database is null)
            return;

        var authors = database.Authors.ToList();
        context.Response.StatusCode = 200;
        await context.Response.WriteAsJsonAsync(authors);
    }

    internal static async Task EditAuthor(HttpContext context)
    {
        var app = AppTools.GetApp();
        
        using var scope = app.Services.CreateScope();
        var database = AppTools.GetDatabase(app.Logger, context, scope);
        if (database is null)
            return;
        
        var request = await AppTools.ParseRequestJson<EditAuthorRequest>(app.Logger, context);
        if (request is null)
            return;

        var target = await database.Authors.SingleOrDefaultAsync(u => u.Id == request.Id);
        if (target is null)
        {
            app.Logger.LogWarning($"The author with id '{request.Id}' was not found.");
            context.Response.StatusCode = 404;
            return;
        }
        
        await AppTools.UpdateInstance(target, request, app.Logger, database, context);
    }
}