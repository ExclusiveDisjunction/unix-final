using System.Text.Json;
using backend.Info;
using Microsoft.AspNetCore.Mvc;
using Microsoft.EntityFrameworkCore;

namespace backend;

public class Program
{
    private static WebApplication? _app;

    private static async ValueTask<User?> GetUserAsync(Database database, string username)
    {
        var query = from user in database.Users
                    where user.Username == username
                    select user;

        return await query.ElementAtAsync(0);
    }

    private static async Task AddGenre(HttpContext context) 
    {
        if (_app is null) 
        {
            await Console.Error.WriteLineAsync("Application is not created.");
            context.Response.StatusCode = 500;
            return;
        }
        
        try {
            var info = await context.Request.ReadFromJsonAsync<AddGenreRequest>();
            if (info is null) {
                _app.Logger.LogError("The provided message is not valid JSON, and could not be represented as a AddGenreRequest.");
                context.Response.StatusCode = 400;
                return;
            }

            using var scope = _app.Services.CreateScope();
            var database = scope.ServiceProvider.GetRequiredService<Database>();

            var genre = new Info.Genre(info.Id, info.Name, info.Description);
            database.Genres.Add(genre);

            context.Response.StatusCode = 200;
        }
        catch (JsonException jsonEx)
        {
            _app.Logger.LogError($"Unable to parse request '{jsonEx}'");
        }
        catch (InvalidOperationException ex) 
        {
            _app.Logger.LogWarning($"The database could not be retrieved, error '{ex}");
        }
    }
    private static async Task AddBook(HttpContext context, string username)
    {
        if (_app is null) 
        {
            await Console.Error.WriteLineAsync("Application is not created.");
            return;
        }

        _app.Logger.LogInformation("Adding book request");
        try
        {
            var info = await context.Request.ReadFromJsonAsync<AddBookRequest>();
            if (info is null)
            {
                _app.Logger.LogError("The request is not valid JSON, or could not be parsed as an AddBookRequest.");
                return;
            }

            /* using var scope = app.Services.CreateScope();
            var database = scope.ServiceProvider.GetRequiredService<Database>();
            database.Books.Add(new Info.Book(info.Id, info.Title));
            */
        } 
        catch (Exception ex)
        {
            _app.Logger.LogError($"Unable to parse book add request, error {ex.Message}");
        }
    }

    private static async Task AddGroup(HttpContext context, string username)
    {
        if (_app is null)
        {
            await Console.Error.WriteLineAsync("Application is not created.");
            return;
        }

        if (username is null || username.Length == 0)
        {
            context.Response.StatusCode = 400;
            return;
        }

        _app.Logger.LogInformation($"Adding group request to user '{username}'");
        Database database;
        try
        {
            using var scope = _app.Services.CreateScope();
            database = scope.ServiceProvider.GetRequiredService<Database>(); 
        }
        catch (InvalidOperationException ex)
        {
            _app.Logger.LogError($"Unable to get the database {ex}");
            context.Response.StatusCode = 400;
            return;
        }

        AddGroupRequest? request;
        try
        {
            _app.Logger.LogInformation($"hopefully this works? {context.Request.Body.ToString()}");
            request = await context.Request.ReadFromJsonAsync<AddGroupRequest>();
        }
        catch
        {
            request = null;
        }

        if (request is null)
        {
            _app.Logger.LogWarning($"Unable to parse JSON content into a AddGroupRequest");
            context.Response.StatusCode = 400;
            return;
        }

        var user = await GetUserAsync(database, username);
        if (user is null)
        {
            _app.Logger.LogWarning("The user could not be found.");
            context.Response.StatusCode = 404;
            return;
        }

        var newGroup = new Info.Group(user.Username, request.Name, request.Description);
        await database.Groups.AddAsync(newGroup);

        context.Response.StatusCode = 200;
    }

    private static async Task GetGenres(HttpContext context)
    {
        if (_app is null)
        {
            await Console.Error.WriteLineAsync("Application is not created.");
            return;
        }

        context.Response.StatusCode = 501;
    }

    private static async Task SignIn(HttpContext context)
    {
        if (_app is null)
        {
            await Console.Error.WriteLineAsync("Application is not created.");
            return;
        }

        context.Response.StatusCode = 501;
    }

    private static async Task CreateUser(HttpContext context)
    {
        if (_app is null)
        {
            await Console.Error.WriteLineAsync("Application is not created.");
            return;
        }

        context.Response.StatusCode = 501;
    }

    private static async Task SignOut(HttpContext context)
    {
        if (_app is null)
        {
            await Console.Error.WriteLineAsync("Application is not created.");
            return;
        }

        context.Response.StatusCode = 501;
    }

    private static async Task GetGroups(HttpContext context, string username)
    {
        if (_app is null)
        {
            await Console.Error.WriteLineAsync("Application is not created.");
            return;
        }

        context.Response.StatusCode = 501;
    }

    private static async Task GetAuthors(HttpContext context)
    {
        if (_app is null)
        {
            await Console.Error.WriteLineAsync("Application is not created.");
            return;
        }

        context.Response.StatusCode = 501;
    }
    private static async Task GetBooks(HttpContext context, string username)
    {
        if (_app is null)
        {
            await Console.Error.WriteLineAsync("Application is not created.");
            context.Response.StatusCode = 500;
            return;
        }

        _app.Logger.LogInformation($"Getting books for {username}");
        
        using var scope = _app.Services.CreateScope();
        try
        {
            var database = scope.ServiceProvider.GetRequiredService<Database>();

            var selectedUser = await GetUserAsync(database, username);
            if (selectedUser is null)
            {
                _app.Logger.LogWarning("The user could not be found.");
                context.Response.StatusCode = 404;
                return;
            }

            var groups = selectedUser.Groups;
            var books = new List<Book>();
            foreach (var group in groups)
            {
                books.AddRange(group.Books);
            }

            await context.Response.WriteAsJsonAsync<List<Book>>(books);
            context.Response.StatusCode = 200;
        }
        catch (InvalidOperationException ex)
        {
            _app.Logger.LogError($"The database could not be accessed '{ex}'");
            context.Response.StatusCode = 500;
        }
        catch (ArgumentOutOfRangeException ex)
        {
            _app.Logger.LogError($"The database did not include a '{username}' user. Error: {ex}");
            context.Response.StatusCode = 400;
        }
    }

    private static WebApplicationBuilder MakeBuilder(string[] args) {
        var builder = WebApplication.CreateBuilder(args);

        // Add all needed services. Use the custom database class.
        builder.Services.AddOpenApi();
        builder.Services.AddDbContext<Database>();
        builder.Services.AddControllers();

        return builder;
    }

    public static void Main(string[] args)
    {
        var builder = MakeBuilder(args);
        _app = builder.Build();

        // Configure the HTTP request pipeline.
        if (_app.Environment.IsDevelopment())
        {
            _app.MapOpenApi();
        }

        //app.UseHttpsRedirection();

        _app.MapPost("/sign-in/", SignIn).WithName("SignIn");
        _app.MapPost("/create-user/", CreateUser).WithName("CreateUser");
        
        _app.MapPost("/{username}/add-book/", AddBook).WithName("AddBook");
        _app.MapPost("/{username}/add-group/", AddGroup).WithName("AddGroup");
        _app.MapPost("/add-genre/", AddGenre).WithName("AddGenre");
        
        _app.MapGet("/{username}/books", GetBooks).WithName("GetBooks");
        _app.MapGet("/genres/", GetGenres).WithName("GetGenres");
        _app.MapGet("/{username}/groups/", GetGroups).WithName("GetGroups");
        _app.MapGet("/authors/", GetAuthors).WithName("GetAuthors");
        
        _app.Run();
    }
}