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

    protected override void OnConfiguring(DbContextOptionsBuilder optionsBuilder)
    {
        string password;
        string? hostName;
        try 
        {   
            var path = Environment.GetEnvironmentVariable("DB_PASSWORD");
            hostName = Environment.GetEnvironmentVariable("DB_HOST");
            if (path is null || hostName is null)
            {
                Console.Error.WriteLine("Unable to read DB_PASSWORD and/or DB_HOST");
                throw new Exception("Unable to open the database password.");
            }
            
            var binary = File.ReadAllBytes(path);

            password = System.Text.Encoding.UTF8.GetString(binary);
        }
        catch (FileNotFoundException e)
        {
            Console.Error.WriteLine($"Unable to find database key file, error: {e}.");
            throw new Exception("Unable to open the database password.");
        }
        
        optionsBuilder.UseNpgsql($"Host={hostName};Username=postgres;Password={password}");
    }

    protected override void OnModelCreating(ModelBuilder modelBuilder)
    {
        modelBuilder.Entity<User>()
            .HasKey(u => u.Username);
        modelBuilder.Entity<User>()
            .Property(u => u.Username)
            .IsRequired()
            .HasMaxLength(50);
        modelBuilder.Entity<User>()
            .Property(u => u.FirstName)
            .IsRequired()
            .HasMaxLength(50);
        modelBuilder.Entity<User>()
            .Property(u => u.LastName)
            .HasMaxLength(50)
            .IsRequired();
        modelBuilder.Entity<User>()
            .Property(u => u.PasswordHash)
            .HasMaxLength(512)
            .IsRequired();

        modelBuilder.Entity<Author>()
            .HasKey(u => u.Id);
        modelBuilder.Entity<Author>()
            .Property(u => u.FirstName)
            .IsRequired()
            .HasMaxLength(50);
        modelBuilder.Entity<Author>()
            .Property(u => u.LastName)
            .IsRequired()
            .HasMaxLength(50);
        modelBuilder.Entity<Author>()
            .HasIndex(u => new { u.FirstName, u.LastName })
            .IsUnique();

        modelBuilder.Entity<Genre>()
            .HasKey(g => g.Name);
        modelBuilder.Entity<Genre>()
            .Property(g => g.Name)
            .IsRequired()
            .HasMaxLength(50);
        modelBuilder.Entity<Genre>()
            .Property(g => g.Description)
            .HasMaxLength(250);
        modelBuilder.Entity<Genre>()
            .HasMany(u => u.Books)
            .WithMany(u => u.Genres);

        modelBuilder.Entity<Group>()
            .HasOne(u => u.Parent)
            .WithMany(p => p.Groups)
            .HasForeignKey(u => u.ParentId);
        modelBuilder.Entity<Group>()
            .HasIndex(u => new { u.Name, u.ParentId })
            .IsUnique();
        modelBuilder.Entity<Group>()
            .HasKey(u => u.Id);
        modelBuilder.Entity<Group>()
            .Property(u => u.Name)
            .HasMaxLength(50);
        modelBuilder.Entity<Group>()
            .Property(u => u.Description)
            .HasMaxLength(250);
        modelBuilder.Entity<Group>()
            .Property(u => u.ParentId)
            .HasMaxLength(50);
        
        modelBuilder.Entity<Book>()
            .HasOne(u => u.Author)
            .WithMany(a => a.Books)
            .HasForeignKey(u => u.AuthorId);
        modelBuilder.Entity<Book>()
            .HasOne(u => u.Group)
            .WithMany(g => g.Books)
            .HasForeignKey(u => u.GroupId);
        modelBuilder.Entity<Book>()
            .Property(u => u.Title)
            .HasMaxLength(50);
        modelBuilder.Entity<Book>()
            .HasIndex(u => new { u.GroupId, u.Title })
            .IsUnique();

    }
}