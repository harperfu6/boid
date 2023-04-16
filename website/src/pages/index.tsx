import init, { World } from "@/pkg/boid";
import { Graphics, Stage } from "@pixi/react";
import { useCallback, useEffect, useState } from "react";

type BoidProps = {};

const Boid: React.FC<BoidProps> = () => {
  const draw = useCallback((g: any) => {
    g.clear;
    g.beginFill(0xff3300);
    g.lineStyle(4, 0xffd900, 1);
    g.moveTo(50, 50);
    g.lineTo(250, 50);
    g.lineTo(100, 100);
    g.lineTo(50, 50);
    g.endFill();
  }, []);
  return <Graphics draw={draw} />;
};

const Home = () => {
  const [loadWasm, setLoadWasmFlg] = useState(false);

  useEffect(() => {
    init()
      .then(() => setLoadWasmFlg(true))
      .then((err) => console.log("err", err));
  });

  useEffect(() => {
    if (!loadWasm) return;
  }, [loadWasm]);

  if (!loadWasm) return <div>loading wasm...</div>;

  const numBoids = 250;
  const size = 600;
  const theFlock = World.new(numBoids, size);
	console.log(theFlock);

  return (
    <>
      <Stage width={600} height={300} options={{ backgroundColor: 0xffffff }}>
        <Boid />
      </Stage>
    </>
  );
};

export default Home;
