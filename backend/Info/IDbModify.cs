namespace backend.Info;

public interface IDbConstructable<T, TN> where T : IDbConstructable<T, TN>
{
    static abstract T CreateFrom(TN source, Database database);
}

public interface IUpdatableRecord
{
    bool Keep { get; }
}
public interface IDbUpdatable<TE> where TE: IUpdatableRecord
{ 
    void UpdateFrom(TE source, Database database);
}

public interface ISimpleRepresentable<out TD>
{
    TD GenerateData();
}

public interface IDbModify<T, TN, TE, out TD> :
    IDbConstructable<T, TN>,
    IDbUpdatable<TE>,
    ISimpleRepresentable<TD>
    where T : IDbModify<T, TN, TE, TD>
    where TE: IUpdatableRecord;