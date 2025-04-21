import {useState, useEffect} from 'react'
import { Link, useNavigate } from 'react-router-dom'
import { LibraryBig, BookCopy, BookOpen, House, CirclePlus } from "lucide-react"

export default function Navbar() {
    const[scrolled, setScrolled]= useState(false)

    useEffect(()=>{
        const handleScroll=() => {
            setScrolled(window.scrollY>20)
        }

        window.addEventListener('scroll', handleScroll)
        return() => window.removeEventListener('scroll', handleScroll)
    }, [])

    const navItems = [
        {name: "Dashboard", href: "/#dashboard", icon: <House className="h-4 w-4" />},
        {name: "Books", href: "/#books", icon: <LibraryBig className="h-4 w-4" />},
        {name: "Collections", href: "/#collections", icon: <BookCopy className="h-4 w-4" />},
    ]

    return (
        <nav 
            className= {`fixed top-0 left-0 right-0 z-50 transition-all duration-300 ${
                scrolled ? "bg-black/80 backdrop-blur-md" : "bg-transparent"
        }`}
        >
            <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div className="flex items-center justify-between h-16">
                    {/* Logo */}
                    <div className="flex-shrink-0 flex items-center">
                        <Link to="/dashboard" className="group flex items-center">
                            <div className="relative h-8 w-8 mr-2">
                                <div className="absolute inset-0 bg-white-500 rounded-md blur-[2px] group-hover:blur-[4px] transition-all"></div>
                                <div className="absolute inset-0 bg-black rounded-md flex items-center justify-center">
                                    <BookOpen className="h-5 w-5 text-white-400" />
                                </div>
                            </div>
                            <span className="text-white font-bold text-xl tracking-wider">
                                PRIVATE<span className="text-#22aa64-400">LIBRARY</span>
                            </span>
                        </Link>
                    </div>
                    <div className="flex items-center space-x-8">
                        {navItems.map((item) => (
                            <Link
                                key={item.name}
                                href={item.href}
                                className="group relative px-3 py-2 text-sm font-medium text-gray-300 hover:text-white transition-colors"
                            >
                                <span className="flex items-center gap-2">
                                    {item.icon}
                                    {item.name}
                                </span>
                                <span className="absolute bottom-0 left-0 h-[1px] w-0 bg-gradient-to-r from-#22aa64-400 to-#38d668-500 transition-all duration-300 group-hover:w-full"></span>
                            </Link>
                        ))}
                        <button className="ml-4 px-4 py-2 bg-gradient-to-r from-#22aa64-500 to-#22aa64-500 text-white font-medium rounded-md hover:shadow-[0_0_15px_rgba(6,182,212,0.5)] transition-shadow flex items-center">
                            Add Book <CirclePlus className="ml-1 h-4 w-4" />
                        </button>
                    </div>
                </div>
            </div>
            <div className="absolute bottom-0 left-0 right-0 h-[1px] bg-gradient-to-r from-transparent via-#22aa64-500/50 to-transparent"></div>
            <div className="absolute top-0 left-0 w-24 h-[1px] bg-gradient-to-r from-#22aa64-500 to-transparent"></div>
            <div className="absolute top-0 right-0 w-24 h-[1px] bg-gradient-to-l from-white-500 to-transparent"></div>
    </nav>
    )
}






