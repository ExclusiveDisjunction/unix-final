import Navbar from "../components/navbar"

export default function DashboardPage() {
    return (
    <div>
        <Navbar />
        <main className="pt-16 px-4">
            <h1 className="text-2xl font-bold text-white">Welcome to the Dashboard</h1>
          {/* Your other dashboard content here */}
        </main>
    </div>  
    )
}
