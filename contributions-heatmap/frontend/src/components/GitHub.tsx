import GitHubLogoLarge from '../../public/logo_large.svg';
import Issues from '../../public/issues.svg';
import PullRequests from '../../public/pullrequests.svg';
import Notifications from '../../public/notifications.svg';
import Overview from '../../public/overview.svg';
import Repositories from '../../public/repositories.svg';
import Projects from '../../public/projects.svg';
import Packages from '../../public/packages.svg';
import Stars from '../../public/stars.svg';
import Followers from '../../public/followers.svg';
import Location from '../../public/location.svg';
import HyperLink from '../../public/link.svg';
import Commit from '../../public/commit.svg';
import Flame from '../../public/flame.svg';
import GitHubLogo from '../../public/logo.svg';

import MagnifyingGlass from "../../public/magnifying_glass.svg";
import CoPilot from "../../public/copilot.svg";
import DropDown from "../../public/dropdown.svg";
import Plus from "../../public/plus.svg";
import Hamburger from "../../public/hamburger.svg";


export function Separator() {
    return <div className="w-px bg-gray-300 h-full">&nbsp;</div>
}

export function SearchBar() {
    return <div className="border-1 border-gray-300 p-1 rounded flex items-center">
        <MagnifyingGlass/>
        <input type="text" placeholder=" Type / to search"/>
    </div>;
}

export function CoPilotCluster() {
    return <div className="flex justify-between space-x-1 items-center border-1 border-gray-300 p-2 rounded">
        <CoPilot/>
        <DropDown/>
    </div>
}

export function CreateCluster() {
    return <div className="flex justify-between space-x-2 items-center">
        <Plus/>
        <DropDown/>
    </div>
}

export function Avatar() {
    return <div className="bg-white w-8 h-8 rounded-full border-1 border-gray-300">
        <img src="https://avatars.githubusercontent.com/u/1186602?v=4" alt="" height="32" width="32"/>
    </div>;
}

export function SidePanel() {
    return <div className="border-1 border-gray-300 p-2 rounded"><Hamburger/></div>;
}

export function AppHeaderGlobal() {
    let menuItems = [
        <CreateCluster/>,
        <Issues/>,
        <PullRequests/>,
        <Notifications/>,
    ];
    return <div className="flex justify-between items-center p-4 pb-2">
        <div className="flex items-center space-x-4">
            <SidePanel/>
            <GitHubLogoLarge/>
            <div className="text-sm font-bold">folkol</div>
        </div>
        <div className="flex items-center space-x-2 text-gray-500">
            <SearchBar/>
            <CoPilotCluster/>
            <Separator/>
            {menuItems.map(item =>
                <span className="border-1 border-gray-300 p-2 rounded">{item}</span>
            )}
            <Avatar/>
        </div>
    </div>;
}

export function AppHeaderLocal() {
    let menuItems = [
        ['Overview', <Overview/>],
        ['Repositories', <Repositories/>, 207],
        ['Projects', <Projects/>],
        ['Packages', <Packages/>],
        ['Stars', <Stars/>, 21],
    ] as const;
    return <div className="pl-4 border-b-1 border-gray-300">
        <ul className="flex space-x-4">
            {menuItems.map(([name, icon, number]) => <li
                className="flex items-center space-x-2 border-red-300 first:border-b-2 p-2">
                <span className="text-gray-500">{icon}</span>
                <span>{name}</span>
                {number && <span className="ml-1 bg-gray-200 px-2 rounded-2xl text-sm">{number}</span>}
            </li>)
            }
        </ul>
    </div>;
}

export function Profile() {
    return <div className="flex flex-col space-y-2 text-gray-700 mb-4 mt-4">
        <div className="bg-white rounded-full border-1 border-gray-300">
            <img width="100%" alt="" src="https://avatars.githubusercontent.com/u/1186602?v=4"/>
        </div>
        <p className="mt-4 mb-0 text-2xl font-bold">Matte</p>
        <p className="mb-3 text-xl text-gray-500">folkol</p>
        <p>This GitHub account is mainly for educational and/or recreational hacks of mine.</p>
        <button type="button"
                className="w-full bg-gray-100 border border-gray-300 rounded p-1 mt-2">
            Edit profile
        </button>
        <div className="flex items-center">
            <Followers className="mr-1"/>
            <span><span className="font-bold">12</span> followers · <span
                className="font-bold">11</span> following</span></div>
        <div className="flex items-center text-sm"><Location/><span className="ml-2">Stockholm</span></div>
        <div className="flex items-center text-sm"><HyperLink/><span className="ml-2">https://folkol.com</span></div>
    </div>;
}

function makeLang(lang: string) {
    let backgroundColor = lang === 'Rust' ? '#dea584' : '#f1e05a';
    return <span className="flex items-center gap-2">
        <div style={{backgroundColor}} className="border rounded-full w-3 h-3 border-gray-500 mix-blend-multiply"/>
        {lang}
    </span>
}

function makeRepoCard([name, desc, lang, n]: string[]) {
    return <div className="border rounded border-gray-300 p-4 w-full text-gray-500 text-sm flex flex-col gap-2">
        <div className="flex gap-2">
            <Commit/>
            <span className="text-blue-500 font-bold text-sm">{name}</span>
            <span className="border rounded-2xl px-2 border-gray-300">Public</span>
        </div>
        <div className="text-gray-500 text-xs">{desc}</div>
        <div className="flex items-center gap-2">
            <div>{makeLang(lang)}</div>
            <div className="flex items-center"><Stars/>
                <div>{n}</div>
            </div>
        </div>
    </div>
}

export function Pinned() {
    let repos = [
        ['futils', 'Various utility programs', 'Rust', '1'],
        ['criterion.js', 'Partial JavaScript-port of Criterion.rs', 'JavaScript', '1'],
    ];
    return <div>
        <p>Pinned</p>
        <div className="flex gap-4">{repos.map(makeRepoCard)}</div>
    </div>;
}

function makeEventCard([icon, desc]: any[]) {
    return <div className="ml-6">
        <div className="p-6 border-l-2 border-gray-300 flex gap-2 items-center">
            <div
                className="ml-[-41px] border-white border-2 bg-gray-100 w-8 h-8 flex items-center justify-center rounded-full">
                {icon}</div>
            {desc}
        </div>
    </div>
}

function makeYear(year: number) {
    return <li
        className="first:bg-blue-500 first:text-white p-2 rounded text-gray-500 text-sm mb-1 not-first:hover:bg-gray-100">
        {year}
    </li>;
}

export function History() {
    let years = Array.from({length: 15}, (_, x) => 2025 - x);
    return <div>
        <ul className="ml-4">
            {years.map(makeYear)}
        </ul>
    </div>;
}

export function ContributionActivity() {
    let events = [
        [<Commit/>, <p>Created 12 commits in 3 repositories</p>],
        [<Flame/>, <p>Created a pull request in <span className="underline">folkol/tutorials</span> that received 1
            comment
        </p>],
        [<PullRequests/>, <p>Opened 2 other pull requests in 2 repositories</p>],
    ];
    return <div>
        <h2 className="py-6">Contribution activity</h2>
        <div className="border-b-1 border-gray-300 h-3 text-sm">
            <span className="bg-white mb-[-1] p-2 pr-3">March <span className="text-gray-500">2025</span></span>
        </div>
        {events.map(makeEventCard)}
        <button className="w-full border border-gray-300 rounded p-1 mt-2 text-xs p-2 font-bold text-blue-500"
                type="button">Show more activity
        </button>
    </div>;
}

export function Footer() {
    let footerItems = [
        '© 2025 GitHub, Inc.',
        'Terms',
        'Privacy',
        'Security',
        'Status',
        'Docs',
        'Contact',
        'Manage cookies',
        'Do not share my personal information',
    ]
    return <footer className="flex justify-center items-center space-x-3 text-sm pt-8 pb-6 text-gray-500">
        <GitHubLogo width="24" height="24"/>
        {footerItems.map(item => <span>{item}</span>)}
    </footer>;
}

function makeBadge(item: string) {
    return <img width="64" height="64" src={item} alt=""/>;
}

export function Achievements() {
    let achievementBadges = [
        "https://github.githubassets.com/assets/pull-shark-default-498c279a747d.png",
        "https://github.githubassets.com/assets/quickdraw-default-39c6aec8ff89.png",
        "https://github.githubassets.com/assets/pair-extraordinaire-default-579438a20e01.png",
        "https://github.githubassets.com/assets/arctic-code-vault-contributor-default-df8d74122a06.png",
    ];
    return <div className="border-t-1 border-gray-200 pt-4">
        <div className="font-bold text-base">Achievements</div>
        <div className="flex flex-wrap">
            {achievementBadges.map(makeBadge)}
        </div>
    </div>;
}