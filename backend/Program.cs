using backend;
using Microsoft.EntityFrameworkCore;
using Npgsql;

public class Program
{
    public static void Main(string[] args)
    {
        var builder = WebApplication.CreateBuilder(args);

// Add services to the container.
// Learn more about configuring OpenAPI at https://aka.ms/aspnet/openapi
        builder.Services.AddOpenApi();

        builder.Services.AddDbContext<Database>();
        builder.Services.AddControllers();

        var app = builder.Build();

// Configure the HTTP request pipeline.
        if (app.Environment.IsDevelopment())
        {
            app.MapOpenApi();
        }

//app.UseHttpsRedirection();

/*
app.MapGet("/weatherforecast", () =>
    {
        var forecast = Enumerable.Range(1, 5).Select(index =>
                new WeatherForecast
                (
                    DateOnly.FromDateTime(DateTime.Now.AddDays(index)),
                    Random.Shared.Next(-20, 55),
                    summaries[Random.Shared.Next(summaries.Length)]
                ))
            .ToArray();
        return forecast;
    })
    .WithName("GetWeatherForecast");
*/

        app.MapPost("/add-book/", async context =>
        {
            app.Logger.LogInformation("Adding book request");
            try
            {
                AddBookRequest? info = await context.Request.ReadFromJsonAsync<AddBookRequest>();
                if (info is null)
                {
                    app.Logger.LogWarning("Book not found");
                    return;
                }

                using var scope = app.Services.CreateScope();
                var database = scope.ServiceProvider.GetRequiredService<Database>();
                database.Books.Add(new backend.Info.Book(info.Id, info.Title));
            }
            catch (Exception ex)
            {
                app.Logger.LogError(ex.Message);
            }
        }).WithName("AddBook");

        app.MapPost("/add-genre/", async context =>
        {

        }).WithName("AddGenre");
        
        app.Run();
    }
}

record AddBookRequest
{
    public int Id { get; init; }
    public string Title { get; init; }
}

record AddGenreRequest
{
    public int Id { get; set; }
    public string Name { get; set; }
    public string Description { get; set; }
}