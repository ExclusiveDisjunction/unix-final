using System.IdentityModel.Tokens.Jwt;
using System.Security.Claims;
using System.Text;
using System.Text.Json;
using Microsoft.IdentityModel.Tokens;

namespace backend;

using Info;
using Microsoft.EntityFrameworkCore;

public class UserMgnt
{
    public static async ValueTask<byte[]?> GetJwtKey()
    {
        try
        {
            return await File.ReadAllBytesAsync("/etc/backend/jwt_key");
        }
        catch
        {
            return null;
        }
    }

    private static Dictionary<string, string> _activeUsers = new(); //JWT -> Username

    public static String? ExtractJwtFromRequest(HttpContext context)
    {
        string? token = null;

        if (context.Request.Headers.TryGetValue("Authorization", out var authHeader))
        {
            var bearerToken = authHeader.ToString();
            if (bearerToken.StartsWith("Bearer ", StringComparison.OrdinalIgnoreCase))
            {
                token = bearerToken.Substring("Bearer ".Length).Trim();
            }
        }

        return token;
    }
    public static bool ValidateJwtAgainstUser(string username, HttpContext context)
    {
        var token = ExtractJwtFromRequest(context);
        if (token is null)
        {
            return false;
        }
        
        _activeUsers.TryGetValue(token, out var lookupUsername);
        return lookupUsername is not null && lookupUsername == username;
    }
    
    public static async ValueTask<User?> GetUserAsync(Database database, string username)
    {
        var query = from user in database.Users
            where user.Username == username
            select user;
        try
        {
            return await query.ElementAtAsync(0);
        }
        catch
        {
            return null;
        }
    }
    public static async ValueTask<string?> GenerateToken(User target)
    {
        var keyValue = await GetJwtKey();
        if (keyValue is null)
        {
            return null;
        }
        
        var key = new SymmetricSecurityKey(keyValue);
        var creds = new SigningCredentials(key, SecurityAlgorithms.HmacSha256);

        var token = new JwtSecurityToken(
            issuer: "dotnet-backend",
            audience: "your_audience",
            claims: new List<Claim>(),
            expires: DateTime.Now.AddMinutes(30),
            signingCredentials: creds);

        var result = new JwtSecurityTokenHandler().WriteToken(token);
        if (result is null)
        {
            return null;
        }

        _activeUsers[result] = target.Username;

        return result;
    }

    internal static async Task GenerateTokenRoute(HttpContext context, string username)
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

        var targetUser = await GetUserAsync(database, username);
        if (targetUser is null)
        {
            Program.App.Logger.LogWarning("The specified user could not be found.");
            context.Response.StatusCode = 404;
            return;
        }
        
        var token = await GenerateToken(targetUser);
        if (token is null)
        {
            Program.App.Logger.LogWarning("Unable to generate a JWT token.");
            context.Response.StatusCode = 500;
            return;
        }
        
        context.Response.StatusCode = 200;
        await context.Response.WriteAsync(token);
    }
    
    internal static async Task SignIn(HttpContext context)
    {
        if (Program.App is null)
        {
            await Console.Error.WriteLineAsync("Application is not created.");
            return;
        }

        context.Response.StatusCode = 501;
    }

    internal static async Task CreateUser(HttpContext context)
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

        CreateUserRequest? request;
        try
        {
            request = await context.Request.ReadFromJsonAsync<CreateUserRequest>();
        }
        catch (JsonException ex)
        {
            Program.App.Logger.LogWarning($"Invalid JSON syntax: '{ex}");
            request = null;
        }

        if (request is null)
        {
            Program.App.Logger.LogWarning("The JSON could not be parsed into a CreateUserRequest");
            context.Response.StatusCode = 400;
            return;
        }

        var username = request.Username;
        if (await GetUserAsync(database, username) is not null)
        {
            Program.App.Logger.LogInformation($"User '{username}' is already registered.");
            context.Response.StatusCode = 409;
        }
        
        
    }

    internal static async Task SignOut(HttpContext context, string username)
    {
        if (Program.App is null)
        {
            await Console.Error.WriteLineAsync("Application is not created.");
            return;
        }

        context.Response.StatusCode = 501;
    }
}