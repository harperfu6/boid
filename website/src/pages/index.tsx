import init, { World, Boid } from "@/pkg/boid";
import { Graphics, Stage } from "@pixi/react";
import { useCallback, useEffect, useState } from "react";

type BoidItemProps = {
  x: number;
  y: number;
};

const BoidItem: React.FC<BoidItemProps> = ({ x, y }) => {
  const draw = useCallback(
    (g: any) => {
      g.clear();
      g.beginFill(0x000000);
      g.lineTo(5, 5);
      g.lineTo(10, 0);
      g.lineTo(5, 15);
      g.lineTo(0, 0);
      g.setTransform(x, y);
      g.endFill();
    },
    [x, y]
  );
  return <Graphics draw={draw} />;
};

const Home = () => {
  const [loadWasm, setLoadWasmFlg] = useState(false);
  const [boids, setBoids] = useState<Boid[]>();
  const [flock, setFlock] = useState<World>();
  const numBoids = 250;
  const size = 600;

  useEffect(() => {
    init()
      .then(() => setLoadWasmFlg(true))
      .then(() => {
        const theFlock = World.new(numBoids, size);
        setFlock(theFlock);
      });
  }, []);

  useEffect(() => {
    let timerId: number;

    const step = () => {
      flock?.step(1.1);
      setBoids(flock?.get_boids());
      timerId = requestAnimationFrame(step);
    };

    timerId = requestAnimationFrame(step);

    return () => {
      cancelAnimationFrame(timerId);
    };
  });

  if (!loadWasm) return <div>loading wasm...</div>;
  if (flock === undefined) return <div>loading flock...</div>;

  return (
    <>
      <Stage width={800} height={800} options={{ backgroundColor: 0xffffff }}>
        {boids?.map((b: Boid) => {
          const x = b.point.x;
          const y = b.point.y;
          return <BoidItem key={b.id} x={x} y={y} />;
        })}
      </Stage>
    </>
  );
};

export default Home;
