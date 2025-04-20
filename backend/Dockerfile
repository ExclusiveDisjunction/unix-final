FROM mcr.microsoft.com/dotnet/aspnet:9.0 AS base
USER $APP_UID
WORKDIR /app
EXPOSE 8080
EXPOSE 8081

FROM mcr.microsoft.com/dotnet/sdk:9.0 AS build
ARG BUILD_CONFIGURATION=Release
WORKDIR /src

USER root
RUN apt-get update && apt-get install -y python3
RUN mkdir -p /etc/backend
COPY key_gen.py /etc/backend/
WORKDIR /etc/backend
RUN python3 key_gen.py
RUN --mount=type=secret,id=postgres_password,target=/etc/backend/db_pass
RUN chmod 644 jwt_key


WORKDIR /src
COPY ["backend.csproj", "./"]
RUN dotnet restore "backend.csproj"
COPY . .
WORKDIR "/src/"
RUN dotnet build "./backend.csproj" -c $BUILD_CONFIGURATION -o /app/build

FROM build AS publish
ARG BUILD_CONFIGURATION=Release
RUN dotnet publish "./backend.csproj" -c $BUILD_CONFIGURATION -o /app/publish /p:UseAppHost=false

RUN dotnet tool install --global dotnet-ef
RUN dotnet ef database update --project "backend.csproj"

FROM base AS final
USER root
WORKDIR /app
RUN mkdir -p /etc/backend
COPY --from=publish /app/publish .
COPY --from=build /etc/backend/jwt_key /etc/backend/jwt_key

ENTRYPOINT ["dotnet", "backend.dll"]
