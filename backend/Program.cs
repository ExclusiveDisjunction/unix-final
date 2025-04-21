using Microsoft.AspNetCore.Authentication.JwtBearer;
using Microsoft.EntityFrameworkCore;
using Microsoft.IdentityModel.Tokens;

namespace backend;

public static class Program
{
    internal static WebApplication? App;
    
    private static async Task GreetingsMessage(HttpContext context)
    {
        if (App is null)
        {
            return;
        }
        
        App.Logger.LogInformation("Sending greeting!");
        var response = new Dictionary<string, string>
        {
            ["message"] = "Hello!"
        };

        context.Response.StatusCode = 200;
        await context.Response.WriteAsJsonAsync(response);
    }

    private static async Task<WebApplicationBuilder> MakeBuilder(string[] args) {
        var builder = WebApplication.CreateBuilder(args);

        // Add all needed services. Use the custom database class.
        builder.Services.AddOpenApi();
        builder.Services.AddDbContext<Database>();
        builder.Services.AddControllers();
        
        var keyValue = await UserManagement.GetJwtKey();
        if (keyValue is null)
        {
            throw new Exception("The JWT key value is not found.");
        }
        
        builder.Services.AddAuthentication(JwtBearerDefaults.AuthenticationScheme)
            .AddJwtBearer(JwtBearerDefaults.AuthenticationScheme,
                options =>
                {
                    options.TokenValidationParameters = new TokenValidationParameters
                    {
                        ValidIssuer = "dotnet-backend",
                        ValidAudience = "your_audience",
                        IssuerSigningKey = new SymmetricSecurityKey(keyValue)
                    };
                });
        builder.Services.AddAuthorization();

        return builder;
    }

    public static void Main(string[] args)
    {
        try
        {
            var builder = MakeBuilder(args).Result;
            App = builder.Build();
        }
        catch (Exception ex)
        {
            Console.Error.WriteLine(ex.Message);
            throw;
        }

        using (var scope = App.Services.CreateScope())
        {
            var db = scope.ServiceProvider.GetRequiredService<Database>();
            db.Database.Migrate();
        }

        // Configure the HTTP request pipeline.
        if (App.Environment.IsDevelopment())
        {
            App.MapOpenApi();
        }
        
        App.UseAuthentication();
        App.UseAuthorization();

        //app.UseHttpsRedirection();
        
        //Expects SignInRequest, responds UserInformation?
        App.MapPost("/sign-in/", UserManagement.SignIn)
            .WithName("SignIn");
        //Expects CreateUserRequest, responds UserInformation?
        App.MapPost("/create-user/", UserManagement.CreateUser)
            .WithName("CreateUser"); 
        //Expects EditUserRequest, responds Ok/Conflict
        App.MapPost("/modify_user/{username}", UserManagement.ModifyUser)
            .WithName("ModifyUser")
            .RequireAuthorization();
        
        // Expects JWT in header 'Authorization' & AddBookRequest, responds with http code
        App.MapPost("/{username}/add-book/", BookManagement.AddBook)
            .WithName("AddBook")
            .RequireAuthorization();
        // Expects JWT in header 'Authorization', responds [Book]?
        App.MapGet("/{username}/books", BookManagement.GetBooks)
            .WithName("GetBooks")
            .RequireAuthorization();
        // Expects JWT in header 'Authorization' & EditBookRequest, responds with http code
        App.MapPost("/{username}/add-book/", BookManagement.EditBook)
            .WithName("EditBook")
            .RequireAuthorization();
        // Expects JWT in header 'Authorization' & AuthorData, responds (Ok & AuthorID (int))/Conflict
        App.MapPost("/add-author", BookManagement.AddAuthor)
            .WithName("AddAuthor")
            .RequireAuthorization();
        // Expects JWT in header 'Authorization', responds [Author]?
        App.MapGet("/authors/", BookManagement.GetAuthors)
            .WithName("GetAuthors")
            .RequireAuthorization();
        // Expects JWT in header 'Authorization' & EditAuthorRequest, responds Ok/Conflict
        App.MapPost("/edit-author", BookManagement.EditAuthor)
            .WithName("EditAuthor")
            .RequireAuthorization();
        
        // Expects JWT in header 'Authorization' & AddGroupRequest, responds with HTTP code.
        App.MapPost("/{username}/add-group/", OrgManagement.AddGroup)
            .WithName("AddGroup")
            .RequireAuthorization();
        // Expects JWT in header 'Authorization', responds [Group]?
        App.MapGet("/{username}/groups/", OrgManagement.GetGroups)
            .WithName("GetGroups")
            .RequireAuthorization();
        // Expects JWT in header 'Authorization' & EditOrganizationRequest, Responds Ok/Conflict
        App.MapPost("/{username}/edit-group", OrgManagement.EditGroup)
            .WithName("ModifyGroup")
            .RequireAuthorization();
        // Expects JWT in header 'Authorization' & EditOrganizationRequest, Responds Ok/Conflict
        App.MapPost("/edit-genre", OrgManagement.EditGenre)
            .WithName("EditGenre")
            .RequireAuthorization();
        // Expects JWT in header 'Authorization' & AddGenreRequest, responds with HTTP code.
        App.MapPost("/add-genre/", OrgManagement.AddGenre)
            .WithName("AddGenre")
            .RequireAuthorization();
        // Expects JWT in header 'Authorization', responds with [Genre]?
        App.MapGet("/genres/", OrgManagement.GetGenres)
            .WithName("GetGenres")
            .RequireAuthorization();

        // Expects nothing, returns JSON message
        App.MapGet("/", GreetingsMessage).WithName("Greeting");
        App.MapGet("/auth", GreetingsMessage).WithName("GreetingAuth").RequireAuthorization();
        
        App.Run();
    }
}