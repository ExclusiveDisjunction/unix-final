using backend;
using Microsoft.EntityFrameworkCore;
using Microsoft.Extensions.ObjectPool;
using Npgsql;

namespace backend;

public class Program
{
    private static WebApplication? app = null;

    private static async void AddGenre(HttpContext context) 
    {
        if (app is null) 
        {
            Console.Error.WriteLine("Application is not created.");
            return;
        }

        app.Logger.LogInformation("Adding genre request");
        try {
            AddGenreRequest? info = await context.Request.ReadFromJsonAsync<AddGenreRequest>();
            if (info is null) {
                app.Logger.LogWarning("The provided logger is invalid");
                return;
            }

            using var scope = app.Services.CreateScope();
            var database = scope.ServiceProvider.GetRequiredService<Database>();

            var genre = new Info.Genre(info.Id, info.Name, info.Description);
            database.Genres.Add(genre);
        }
        catch (Exception ex) 
        {
            app.Logger.LogWarning($"Unable to parse request to add genre, error {ex}");
        }
    }
    private static async void AddBook(HttpContext context)
    {
        if (app is null) 
        {
            Console.Error.WriteLine("Application is not created.");
            return;
        }

        app.Logger.LogInformation("Adding book request");
        try
        {
            AddBookRequest? info = await context.Request.ReadFromJsonAsync<AddBookRequest>();
            if (info is null)
            {
                app.Logger.LogWarning("Book not found");
                return;
            }

            /* using var scope = app.Services.CreateScope();
            var database = scope.ServiceProvider.GetRequiredService<Database>();
            database.Books.Add(new Info.Book(info.Id, info.Title));
            */
        } 
        catch (Exception ex)
        {
            app.Logger.LogError($"Unable to parse book add request, error {ex.Message}");
        }
    }

    private static WebApplicationBuilder? MakeBuilder(string[] args) {
        var builder = WebApplication.CreateBuilder(args);

        // Add services to the container.
        // Learn more about configuring OpenAPI at https://aka.ms/aspnet/openapi
        builder.Services.AddOpenApi();

        builder.Services.AddDbContext<Database>();
        builder.Services.AddControllers();

        return builder;
    }

    public static void Main(string[] args)
    {
        var builder = MakeBuilder(args);
        if (builder is null) {
            Console.Error.WriteLine("Unable to create builder.");
            return;
        }

        app = builder.Build();

        // Configure the HTTP request pipeline.
        if (app.Environment.IsDevelopment())
        {
            app.MapOpenApi();
        }

        //app.UseHttpsRedirection();

        app.MapPost("/add-book/", Program.AddBook).WithName("AddBook");

        app.MapPost("/add-genre/", Program.AddGenre).WithName("AddGenre");
        
        app.Run();
    }
}

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