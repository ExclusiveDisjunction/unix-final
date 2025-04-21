using System.IdentityModel.Tokens.Jwt;
using System.Security.Claims;
using Microsoft.IdentityModel.Tokens;

namespace backend;

using Info;
using Microsoft.EntityFrameworkCore;

public static class UserManagement
{
    public static async ValueTask<byte[]?> GetJwtKey()
    {
        try
        {
            var path = Environment.GetEnvironmentVariable("JWT_TOKEN");
            if (path is null)
                return null;
            
            return await File.ReadAllBytesAsync(path);
        }
        catch
        {
            return null;
        }
    }

    private static readonly Dictionary<string, string> ActiveUsers = new(); //JWT -> Username

    private static string? ExtractJwtFromRequest(HttpContext context)
    {
        string? token = null;

        if (!context.Request.Headers.TryGetValue("Authorization", out var authHeader))
            return token;
        
        var bearerToken = authHeader.ToString();
        if (bearerToken.StartsWith("Bearer ", StringComparison.OrdinalIgnoreCase))
            token = bearerToken.Substring("Bearer ".Length).Trim();

        return token;
    }
    public static bool ValidateJwtAgainstUser(string username, HttpContext context)
    {
        var token = ExtractJwtFromRequest(context);
        if (token is null)
            return false;
        
        ActiveUsers.TryGetValue(token, out var lookupUsername);
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

    private static async ValueTask<string?> GenerateUnboundedToken()
    {
        var keyValue = await GetJwtKey();
        if (keyValue is null)
            return null;
        
        var key = new SymmetricSecurityKey(keyValue);
        var credentials = new SigningCredentials(key, SecurityAlgorithms.HmacSha256);

        var token = new JwtSecurityToken(
            issuer: "dotnet-backend",
            audience: "your_audience",
            claims: new List<Claim>(),
            expires: DateTime.Now.AddMinutes(30),
            signingCredentials: credentials);

        return new JwtSecurityTokenHandler().WriteToken(token);
    }
    private static async ValueTask<string?> GenerateToken(User target)
    {
        var result = await GenerateUnboundedToken();
        if (result is null)
            return null;

        ActiveUsers[result] = target.Username;

        return result;
    }
    
    internal static async Task SignIn(HttpContext context)
    {
        var app = AppTools.GetApp();
        
        using var scope = app.Services.CreateScope();
        var database = AppTools.GetDatabase(app.Logger, context, scope);
        if (database is null)
            return;

        var request = await AppTools.ParseRequestJson<SignInRequest>(app.Logger, context);
        if (request is null)
            return;

        var target = await database.Users.SingleOrDefaultAsync(u => u.Username == request.Username);
        if (target is null)
        {
            app.Logger.LogWarning($"The user with username '{request.Username}' was not found");
            context.Response.StatusCode = 404;
            return;
        }
        
        var incomingPassword = request.Password;
        if (!BCrypt.Net.BCrypt.Verify(incomingPassword, target.PasswordHash))
        {
            app.Logger.LogWarning($"The password '{incomingPassword}' is invalid");
            context.Response.StatusCode = 401;
            return;
        }

        var jwt = await GenerateToken(target);
        if (jwt is null)
        {
            app.Logger.LogWarning($"The jwt could not be generated");
            context.Response.StatusCode = 500;
            return;
        }

        var response = target.GenerateData(jwt);
        context.Response.StatusCode = 200;
        await context.Response.WriteAsJsonAsync(response);
    }

    internal static async Task CreateUser(HttpContext context)
    {
        var app = AppTools.GetApp();
        
        using var scope = app.Services.CreateScope();
        var database = AppTools.GetDatabase(app.Logger, context, scope);
        if (database is null)
            return;

        var request = await AppTools.ParseRequestJson<CreateAccountRequest>(app.Logger, context);
        if (request is null)
            return;

        var target = await database.Users.SingleOrDefaultAsync(u => u.Username == request.Username);
        if (target is not null)
        {
            app.Logger.LogWarning($"A user with that username already exists.");
            context.Response.StatusCode = 409;
            return;
        }

        var newUser = new User
        {
            Username = request.Username,
            FirstName = request.FirstName,
            LastName = request.LastName,
            PasswordHash = BCrypt.Net.BCrypt.HashPassword(request.Password),
            Groups = []
        };

        try
        {
            await database.Users.AddAsync(newUser);
            await database.SaveChangesAsync();
        }
        catch (Exception ex)
        {
            app.Logger.LogWarning($"An error occured creating a user: {ex}");
            context.Response.StatusCode = 500;
        }

        var jwt = await GenerateToken(newUser);
        if (jwt is null)
        {
            app.Logger.LogWarning($"The jwt could not be generated");
            context.Response.StatusCode = 500;
            return;
        }

        var response = newUser.GenerateData(jwt);
        context.Response.StatusCode = 200;
        await context.Response.WriteAsJsonAsync(response);
    }

    internal static async Task ModifyUser(HttpContext context, string username)
    {
        var app = AppTools.GetApp();
        
        using var scope = app.Services.CreateScope();
        var database = AppTools.GetDatabase(app.Logger, context, scope);
        if (database is null)
            return;

        var request = await AppTools.ParseRequestJson<EditUserRequest>(app.Logger, context);
        if (request is null)
            return;
        
        var target = await database.Users.SingleOrDefaultAsync(u => u.Username == username);
        if (target is null)
            return;

        if (!ValidateJwtAgainstUser(username, context))
            return;

        await AppTools.UpdateInstance(target, request, app.Logger, database, context);
    }
}