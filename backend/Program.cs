using System.Text.Json;
using backend.Info;
using Microsoft.AspNetCore.Mvc;
using Microsoft.EntityFrameworkCore;

namespace backend;

public class Program
{
    private static WebApplication? _app;

    private static async Task AddGenre(HttpContext context) 
    {
        if (_app is null) 
        {
            await Console.Error.WriteLineAsync("Application is not created.");
            return;
        }
        
        try {
            var info = await context.Request.ReadFromJsonAsync<AddGenreRequest>();
            if (info is null) {
                _app.Logger.LogWarning("The provided logger is invalid");
                return;
            }

            using var scope = _app.Services.CreateScope();
            var database = scope.ServiceProvider.GetRequiredService<Database>();

            var genre = new Info.Genre(info.Id, info.Name, info.Description);
            database.Genres.Add(genre);
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
                _app.Logger.LogWarning("Book not found");
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
        
    }

    private static async Task<List<Genre>> GetGenres(HttpContext context)
    {
        return [];
    }

    private static async Task<ActionResult<SignInResponse>> SignIn(HttpContext context)
    {
        return new ActionResult<SignInResponse>(new SignInResponse(false, "method not implemented", null));
    }

    private static async Task<SignInResponse> CreateUser(HttpContext context)
    {
        return new SignInResponse(false, "method not implemented", null);
    }

    private static async Task SignOut(HttpContext context)
    {
        
    }

    private static async Task<List<Group>?> GetGroups(HttpContext context, string username)
    {
        return null;
    }

    private static async Task<List<Author>> GetAuthors(HttpContext context, string username)
    {
        return [];
    }
    private static async Task<List<Book>?> GetBooks(HttpContext context, string username)
    {
        if (_app is null)
        {
            await Console.Error.WriteLineAsync("Application is not created.");
            return null;
        }

        _app.Logger.LogInformation($"Getting books for {username}");
        
        using var scope = _app.Services.CreateScope();
        try
        {
            var database = scope.ServiceProvider.GetRequiredService<Database>();

            var query = from user in database.Users
                where user.Username == username
                select user;

            var selectedUser = await query.ElementAtAsync(0);
            var groups = selectedUser.Groups;
            var books = new List<Book>();
            foreach (var group in groups)
            {
                books.AddRange(group.Books);
            }

            return books;
        }
        catch (InvalidOperationException ex)
        {
            _app.Logger.LogError($"The database could not be accessed '{ex}'");
        }
        catch (ArgumentOutOfRangeException ex)
        {
            _app.Logger.LogError($"The database did not include a '{username}' user. Error: {ex}");
        }

        return null;
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