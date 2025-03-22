import Link from 'next/link';

const knownUsers = [
    'folkol'
];

function makeLink(user: string) {
    console.log('user', user);
    let url = `/users/${user}`;
    return <li><Link className="text-blue-500 underline" href={url}>{url}</Link></li>;
}

const HomePage = () => {
    return (
        <div>
            <h1 className="text-3xl font-bold mb-4">Known Users</h1>
            <ul className="list-disc pl-5">{knownUsers.map(makeLink)}</ul>
        </div>
    );
};

export default HomePage;