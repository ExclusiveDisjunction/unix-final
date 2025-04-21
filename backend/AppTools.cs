using System.Text.Json;
using backend.Info;
using Microsoft.EntityFrameworkCore;

namespace backend;

public class AppTools
{
    internal static WebApplication GetApp()
    {
        if (Program.App is null)
        {
            throw new InvalidOperationException("The application is not created");
        }

        return Program.App;
    }

    internal static bool EnforceJwtUserValidation(ILogger logger, HttpContext context, string username)
    {
        if (UserManagement.ValidateJwtAgainstUser(username, context))
            return true;

        logger.LogWarning("The user was found, but it did not match their JWT.");
        context.Response.StatusCode = 403;
        return false;
    }

    internal static Database? GetDatabase(ILogger logger, HttpContext context, IServiceScope scope)
    {
        try
        {
            return scope.ServiceProvider.GetRequiredService<Database>();
        }
        catch (InvalidOperationException ex)
        {
            logger.LogError($"Unable to get database: '{ex}'");
            context.Response.StatusCode = 500;
            return null;
        }
    }

    internal static async Task<T?> ParseRequestJson<T>(ILogger logger, HttpContext context)
    {
        T? request;
        try
        {
            request = await context.Request.ReadFromJsonAsync<T>();
        }
        catch (JsonException ex)
        {
            logger.LogError($"Unable to parse request, error {ex.Message}");
            context.Response.StatusCode = 400;
            request = default;
            return request;
        }

        if (request is not null) return request;

        logger.LogError("The request is not valid JSON, or could not be parsed..");
        context.Response.StatusCode = 400;

        return request;
    }

    internal static async Task UpdateInstance<T, TE>(T target, TE request, ILogger logger, Database database,
        HttpContext context) 
        where TE: IUpdatableRecord
        where T : IDbUpdatable<TE>
    {
        try
        {
            if (request.Keep)
                target.UpdateFrom(request, database);
            else
                database.Remove(target);

            await database.SaveChangesAsync();
            context.Response.StatusCode = 200;
        }
        catch (DbUpdateConcurrencyException ex)
        {
            logger.LogWarning($"Unable to update records '{ex}'");
            context.Response.StatusCode = 409;
        }
        catch (Exception ex)
        {
            logger.LogWarning($"Unable to update entity due to '{ex.Message}'");
            context.Response.StatusCode = 400;
        }
    }
}