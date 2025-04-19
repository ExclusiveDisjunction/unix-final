using Microsoft.EntityFrameworkCore;
using backend.Info;

namespace backend;

public class Database : DbContext
{
    public DbSet<User> Users => Set<User>();
    public DbSet<Author> Authors => Set<Author>();
    public DbSet<Genre> Genres => Set<Genre>();
    public DbSet<Group> Groups => Set<Group>();
    public DbSet<Book> Books => Set<Book>();

    protected async override void OnConfiguring(DbContextOptionsBuilder optionsBuilder)
    {
        string password;
        try 
        {   
            var binary = await File.ReadAllBytesAsync("/etc/backend/db_pass");
            if (binary is null) 
            {
                await Console.Error.WriteLineAsync($"Unable to find database key file.");
                return;    
            }

            password = System.Text.Encoding.UTF8.GetString(binary);
        }
        catch (FileNotFoundException e)
        {
            await Console.Error.WriteLineAsync($"Unable to find database key file, error: {e}.");
            return;
        }

        if (password is null)
        {
            throw new Exception("The database password is not presented.");
        }
        
        optionsBuilder.UseNpgsql($"Host=localhost;Username=postgres;Password={password}");
    }

    protected override void OnModelCreating(ModelBuilder modelBuilder)
    {
        modelBuilder.Entity<User>()
            .HasKey(u => u.Username);
        
        modelBuilder.Entity<Group>()
            .HasOne(u => u.Parent)
            .WithMany(p => p.Groups)
            .HasForeignKey(u => u.ParentId);
        
        modelBuilder.Entity<Book>()
            .HasOne(u => u.Author)
            .WithMany(a => a.Books)
            .HasForeignKey(u => u.AuthorId);
        
        modelBuilder.Entity<Book>()
            .HasOne(u => u.Group)
            .WithMany(g => g.Books)
            .HasForeignKey(u => u.GroupId);
        
        
    }
}