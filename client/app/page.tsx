import { Item } from "@/src/models/Item";

const API_BASE_URL = process.env.NEXT_PUBLIC_API_BASE_URL;

const getItems = async (): Promise<Item[]> => {
  const res = await fetch(`${API_BASE_URL}/items`);

  if (!res.ok) {
    throw new Error(`Failed to fetch items: ${res.statusText}`);
  }

  const items: Item[] = await res.json();
  return items;
};

export default async function Home() {
  const items = await getItems();

  if (!items) {
    return <div>Loading items ... (imagine a nice spinner here)</div>;
  }

  return (
    <div className="grid grid-rows-[20px_1fr_20px] items-center justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20 font-[family-name:var(--font-geist-sans)]">
      <main className="flex flex-col gap-8 row-start-2 items-center sm:items-start">
        Sorted Supermarket List
        {items.map((item) => {
          return (
            <div key={item.id} className="product-item">
              <div>Produkt: {item.product}</div>
              <div>Menge: {item.quantity}</div>
            </div>
          );
        })}
      </main>
    </div>
  );
}
