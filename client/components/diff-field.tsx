import { useContext, ReactNode } from "react";
import { Box, Text, HStack, Badge, useColorModeValue } from "@chakra-ui/react";
import { ExamDiffContext } from "../contexts/exam-diff";

interface DiffFieldProps {
  children: ReactNode;
  currentValue: unknown;
  getDeployedValue: (deployedExam: any) => unknown;
}

export function DiffField({
  children,
  currentValue,
  getDeployedValue,
}: DiffFieldProps) {
  const diffContext = useContext(ExamDiffContext);

  if (!diffContext || !diffContext.isDiffMode) {
    return <>{children}</>;
  }

  const { selectedEnvironment, stagingExam, productionExam, isLoading } =
    diffContext;

  const deployedExam =
    selectedEnvironment === "Staging" ? stagingExam : productionExam;

  const diffBg = useColorModeValue("yellow.900", "yellow.900");
  const diffBorder = useColorModeValue("yellow.500", "yellow.500");

  if (isLoading) {
    return <>{children}</>;
  }

  if (!deployedExam) {
    return (
      <Box position="relative">
        {children}
        <Badge
          position="absolute"
          top="-8px"
          right="-8px"
          colorScheme="green"
          fontSize="xs"
        >
          New
        </Badge>
      </Box>
    );
  }

  const deployedValue = getDeployedValue(deployedExam);
  const hasChanged = !isEqual(currentValue, deployedValue);

  if (!hasChanged) {
    return <>{children}</>;
  }

  return (
    <Box position="relative">
      <Box
        border="2px solid"
        borderColor={diffBorder}
        borderRadius="md"
        p={2}
        bg={diffBg}
        position="relative"
      >
        {children}
        <HStack
          position="absolute"
          top="-12px"
          right="8px"
          bg={diffBg}
          px={2}
          spacing={2}
        >
          <Badge colorScheme="yellow" fontSize="xs">
            Modified
          </Badge>
        </HStack>
      </Box>
      <Box mt={2} p={2} bg="gray.800" borderRadius="md" fontSize="sm">
        <Text color="gray.400" fontWeight="bold" fontSize="xs" mb={1}>
          {selectedEnvironment} value:
        </Text>
        <Text color="gray.300" fontFamily="mono" fontSize="xs">
          {formatValue(deployedValue)}
        </Text>
      </Box>
    </Box>
  );
}

function isEqual(a: unknown, b: unknown): boolean {
  if (a === b) return true;
  if (typeof a !== typeof b) return false;
  if (typeof a === "object" && a !== null && b !== null) {
    return JSON.stringify(a) === JSON.stringify(b);
  }
  return false;
}

function formatValue(value: unknown): string {
  if (value === null || value === undefined) {
    return String(value);
  }
  if (typeof value === "object") {
    return JSON.stringify(value, null, 2);
  }
  return String(value);
}
