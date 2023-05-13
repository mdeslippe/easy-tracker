/**
 * Metadata for the websites root layout.
 */
export const metadata = {
    title: "Easy Tracker",
    description:
        "A web-based utility that enables users to easily monitor the current and historical status of their digital services",
};

/**
 * The root layout for the website.
 *
 * @param props The layout's properties.
 * @returns The root layout.
 */
export default function RootLayout({
    children,
}: {
    children: React.ReactNode;
}) {
    return (
        <html lang="en">
            <body>{children}</body>
        </html>
    );
}
