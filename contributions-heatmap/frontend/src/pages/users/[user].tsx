import {
    Achievements,
    AppHeaderGlobal,
    AppHeaderLocal,
    ContributionActivity,
    Footer,
    Pinned,
    Profile,
    History
} from "@/components/GitHub";

import {useRouter} from 'next/router';

function ContributionCalendar() {
    return <div className="border-2 rounded flex justify-center items-center p-10 text-9xl">???</div>;
}

function Main() {
    return (
        <div className="flex-grow flex justify-center">
            <div className="w-full max-w-[1280px] grid grid-cols-[1fr_3fr] gap-4 p-4">
                <div>
                    <Profile/>
                    <Achievements/>
                </div>
                <div className="flex flex-col gap-4">
                    <Pinned/>
                    <div className="w-full grid grid-cols-[5fr_1fr] gap-4 mt-6">
                        <div>
                            <ContributionCalendar/>
                            <ContributionActivity/>
                        </div>
                        <History/>
                    </div>
                </div>
            </div>
        </div>
    );
}

function Header() {
    return (
        <header className="bg-gray-100">
            <AppHeaderGlobal/>
            <AppHeaderLocal/>
        </header>
    );
}

export default function UserPage() {
    const router = useRouter();
    const {user} = router.query;

    return <div className="flex flex-col h-full">
        <Header/>
        <Main/>
        <Footer/>
    </div>;
}
