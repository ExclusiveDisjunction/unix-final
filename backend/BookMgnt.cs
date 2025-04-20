using backend.Info;

namespace backend;

public class BookMgnt
{
    internal static async Task AddBook(HttpContext context, string username)
    {
        if (Program.App is null) 
        {
            await Console.Error.WriteLineAsync("Application is not created.");
            return;
        }

        Program.App.Logger.LogInformation("Adding book request");
        try
        {
            var info = await context.Request.ReadFromJsonAsync<AddBookRequest>();
            if (info is null)
            {
                Program.App.Logger.LogError("The request is not valid JSON, or could not be parsed as an AddBookRequest.");
                return;
            }

            /* using var scope = app.Services.CreateScope();
            var database = scope.ServiceProvider.GetRequiredService<Database>();
            database.Books.Add(new Info.Book(info.Id, info.Title));
            */
        } 
        catch (Exception ex)
        {
            Program.App.Logger.LogError($"Unable to parse book add request, error {ex.Message}");
        }
    }

    internal static async Task GetAuthors(HttpContext context)
    {
        if (Program.App is null)
        {
            await Console.Error.WriteLineAsync("Application is not created.");
            return;
        }

        context.Response.StatusCode = 501;
    }
    internal static async Task GetBooks(HttpContext context, string username)
    {
        if (Program.App is null)
        {
            await Console.Error.WriteLineAsync("Application is not created.");
            context.Response.StatusCode = 500;
            return;
        }

        Program.App.Logger.LogInformation($"Getting books for {username}");
        Program.App.UseAuthentication();
        
        using var scope = Program.App.Services.CreateScope();
        try
        {
            var database = scope.ServiceProvider.GetRequiredService<Database>();

            var selectedUser = await UserMgnt.GetUserAsync(database, username);
            if (selectedUser is null)
            {
                Program.App.Logger.LogWarning("The user could not be found.");
                context.Response.StatusCode = 404;
                return;
            }

            var groups = selectedUser.Groups;
            var books = new List<Book>();
            foreach (var group in groups)
            {
                books.AddRange(group.Books);
            }

            await context.Response.WriteAsJsonAsync(books);
            context.Response.StatusCode = 200;
        }
        catch (InvalidOperationException ex)
        {
            Program.App.Logger.LogError($"The database could not be accessed '{ex}'");
            context.Response.StatusCode = 500;
        }
        catch (ArgumentOutOfRangeException ex)
        {
            Program.App.Logger.LogError($"The database did not include a '{username}' user. Error: {ex}");
            context.Response.StatusCode = 400;
        }
    }
}