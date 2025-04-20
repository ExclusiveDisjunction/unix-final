using Microsoft.AspNetCore.Authentication.JwtBearer;
using Microsoft.IdentityModel.Tokens;

namespace backend;

public class Program
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
        
        var keyValue = await UserMgnt.GetJwtKey();
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
                        ValidIssuer = "dotnet-backend",          // must match
                        ValidAudience = "your_audience",         // must match
                        IssuerSigningKey = new SymmetricSecurityKey(keyValue),
                    };
                });
            //.AddCookie(CookieAuthenticationDefaults.AuthenticationScheme,
            //    options => builder.Configuration.Bind("CookieSettings", options));
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
            return;
        }

        // Configure the HTTP request pipeline.
        if (App.Environment.IsDevelopment())
        {
            App.MapOpenApi();
        }
        
        App.UseAuthentication();
        App.UseAuthorization();

        //app.UseHttpsRedirection();

        App.MapGet("/generate-token/{username}", UserMgnt.GenerateTokenRoute);
        //Expects SignInRequest, responds UserInformation?
        App.MapPost("/sign-in/", UserMgnt.SignIn)
            .WithName("SignIn");
        //Expects CreateUserRequest, responds UserInformation?
        App.MapPost("/create-user/", UserMgnt.CreateUser)
            .WithName("CreateUser"); 
        // Expects JWT as string, responds with only http code
        App.MapPost("/sign-out/{username}", UserMgnt.SignOut).WithName("SignOut");
        
        // Expects JWT in header 'Authorization & AddBookRequest, responds with http code
        App.MapPost("/{username}/add-book/", BookMgnt.AddBook)
            .WithName("AddBook")
            .RequireAuthorization();
        // Expects JWT in header 'Authorization', responds [Book]?
        App.MapGet("/{username}/books", BookMgnt.GetBooks)
            .WithName("GetBooks")
            .RequireAuthorization();
        // Expects JWT in header 'Authorization, responds [Author]?
        App.MapGet("/authors/", BookMgnt.GetAuthors)
            .WithName("GetAuthors")
            .RequireAuthorization();
        
        // Expects JWT in header 'Authorization & AddGroupRequest, responds with HTTP code.
        App.MapPost("/{username}/add-group/", Organization.AddGroup)
            .WithName("AddGroup")
            .RequireAuthorization();
        // Expects JWT in header 'Authorization, responds [Group]?
        App.MapGet("/{username}/groups/", Organization.GetGroups)
            .WithName("GetGroups")
            .RequireAuthorization();
        // Expects JWT in header 'Authorization & AddGenreRequest, responds with HTTP code.
        App.MapPost("/add-genre/", Organization.AddGenre)
            .WithName("AddGenre")
            .RequireAuthorization();
        // Expects JWT in header 'Authorization, responds with [Genre]?
        App.MapGet("/genres/", Organization.GetGenres)
            .WithName("GetGenres")
            .RequireAuthorization();

        // Expects nothing, returns JSON message
        App.MapGet("/", GreetingsMessage).WithName("Greeting");
        App.MapGet("/auth", GreetingsMessage).WithName("GreetingAuth").RequireAuthorization();
        
        App.Run();
    }
}