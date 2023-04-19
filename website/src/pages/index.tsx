import init, { World } from "@/pkg/boid";
import { Graphics, Stage, Sprite } from "@pixi/react";
import { useCallback, useEffect, useState } from "react";

type BoidItemProps = {
  x: number;
  y: number;
};

{
  /* const BOID_BOD: &'static [[f64; 2]] = &[[5.0, 5.0], [10.0, 0.0], [5.0, 15.0], [0.0, 0.0]]; */
}
const BoidItem: React.FC<BoidItemProps> = ({ x, y }) => {
  const draw = useCallback((g: any) => {
    g.clear();
    g.beginFill(0x000000);
    g.lineTo(5, 5);
    g.lineTo(10, 0);
    g.lineTo(5, 15);
    g.lineTo(0, 0);
    g.setTransform(x, y);
    g.endFill();
  }, [x, y]);
  return <Graphics draw={draw} />;
};

type Boid = {
  x: number;
  y: number;
};

const Home = () => {
  const [loadWasm, setLoadWasmFlg] = useState(false);
  const [isAnimating, setIsAnimating] = useState(false);
  const [boids, setBoids] = useState<Boid[]>();
  const [position, setPosition] = useState<number>(0);

  {/* useEffect(() => { */}
  {/*   init().then(() => setLoadWasmFlg(true)); */}
  {/* }); */}

  useEffect(() => {
		let timerId: number;

    const step = () => {
      setPosition((current) => current + 1);
      timerId = requestAnimationFrame(step);
    };

    timerId = requestAnimationFrame(step);

    return () => {
      cancelAnimationFrame(timerId);
    };
  });

  {/* if (!loadWasm) return <div>loading wasm...</div>; */}

  const numBoids = 250;
  const size = 600;
  {/* const theFlock = World.new(numBoids, size); */}

  {
    /* const boids = theFlock.get_boids() */
  }

  return (
    <>
      <Stage width={800} height={800} options={{ backgroundColor: 0xffffff }}>
        <BoidItem x={position} y={position} />
      </Stage>
    </>
  );
};

export default Home;
