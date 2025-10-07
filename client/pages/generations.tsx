import {
  Box,
  Button,
  Center,
  Heading,
  HStack,
  Spinner,
  Stack,
  Text,
  useColorModeValue,
  SimpleGrid,
  Flex,
  Alert,
  AlertIcon,
  AlertTitle,
  AlertDescription,
  useToast,
  NumberInput,
  NumberInputField,
  NumberInputStepper,
  NumberIncrementStepper,
  NumberDecrementStepper,
  FormControl,
  FormLabel,
  FormHelperText,
  Progress,
  Badge,
  Card,
  CardBody,
  CardHeader,
  VStack,
} from "@chakra-ui/react";
import { Sparkles, ArrowLeft } from "lucide-react";
import { useMutation, useQuery } from "@tanstack/react-query";
import { createRoute, useNavigate } from "@tanstack/react-router";
import { useContext, useEffect, useState } from "react";

import { rootRoute } from "./root";
import { getExams, postGenerateExam } from "../utils/fetch";
import { ProtectedRoute } from "../components/protected-route";
import { UsersWebSocketContext } from "../contexts/users-websocket";
import { AuthContext } from "../contexts/auth";
import { landingRoute } from "./landing";
import type { ExamCreatorExam } from "@prisma/client";

interface GenerationProgress {
  examId: string;
  examName: string;
  total: number;
  completed: number;
  failed: number;
  status: "pending" | "in-progress" | "completed" | "failed";
  error?: string;
}

export function Generations() {
  const { user, logout } = useContext(AuthContext)!;
  const { updateActivity } = useContext(UsersWebSocketContext)!;
  const navigate = useNavigate();
  const toast = useToast();

  const [selectedExams, setSelectedExams] = useState<Set<string>>(new Set());
  const [generationCount, setGenerationCount] = useState<number>(1);
  const [generationProgress, setGenerationProgress] = useState<
    Map<string, GenerationProgress>
  >(new Map());
  const [isGenerating, setIsGenerating] = useState(false);

  const examsQuery = useQuery({
    queryKey: ["exams"],
    enabled: !!user,
    queryFn: () => getExams(),
    retry: false,
  });

  const generateExamMutation = useMutation({
    mutationFn: (examId: string) => postGenerateExam(examId),
  });

  function handleExamSelection(examId: string) {
    setSelectedExams((prev) => {
      const newSelection = new Set(prev);
      if (newSelection.has(examId)) {
        newSelection.delete(examId);
      } else {
        newSelection.add(examId);
      }
      return newSelection;
    });
  }

  function handleSelectAll() {
    if (examsQuery.data) {
      setSelectedExams(new Set(examsQuery.data.map(({ exam }) => exam.id)));
    }
  }

  function handleDeselectAll() {
    setSelectedExams(new Set());
  }

  async function handleGenerateExams() {
    if (selectedExams.size === 0 || generationCount < 1) return;

    setIsGenerating(true);
    const examIds = [...selectedExams];
    const progressMap = new Map<string, GenerationProgress>();

    // Initialize progress for each exam
    examIds.forEach((examId) => {
      const exam = examsQuery.data?.find((e) => e.exam.id === examId);
      progressMap.set(examId, {
        examId,
        examName: exam?.exam.config.name || "Unknown Exam",
        total: generationCount,
        completed: 0,
        failed: 0,
        status: "pending",
      });
    });
    setGenerationProgress(progressMap);

    // Generate exams sequentially for each exam
    for (const examId of examIds) {
      const progress = progressMap.get(examId)!;
      progress.status = "in-progress";
      setGenerationProgress(new Map(progressMap));

      // Generate multiple instances for this exam
      for (let i = 0; i < generationCount; i++) {
        try {
          await generateExamMutation.mutateAsync(examId);
          progress.completed += 1;
          setGenerationProgress(new Map(progressMap));
        } catch (error) {
          progress.failed += 1;
          progress.error =
            error instanceof Error ? error.message : "Unknown error";
          setGenerationProgress(new Map(progressMap));
        }
      }

      progress.status =
        progress.failed === 0
          ? "completed"
          : progress.completed > 0
          ? "completed"
          : "failed";
      setGenerationProgress(new Map(progressMap));
    }

    setIsGenerating(false);

    // Show summary toast
    const totalGenerated = Array.from(progressMap.values()).reduce(
      (sum, p) => sum + p.completed,
      0
    );
    const totalFailed = Array.from(progressMap.values()).reduce(
      (sum, p) => sum + p.failed,
      0
    );

    toast({
      title: "Generation Complete",
      description: `Successfully generated ${totalGenerated} exam${
        totalGenerated !== 1 ? "s" : ""
      }${totalFailed > 0 ? `, ${totalFailed} failed` : ""}.`,
      status: totalFailed === 0 ? "success" : "warning",
      duration: 5000,
      isClosable: true,
    });
  }

  function handleReset() {
    setGenerationProgress(new Map());
    setSelectedExams(new Set());
    setGenerationCount(1);
  }

  useEffect(() => {
    updateActivity({
      page: new URL(window.location.href),
      lastActive: Date.now(),
    });
  }, []);

  const bg = useColorModeValue("black", "black");
  const cardBg = useColorModeValue("gray.800", "gray.800");
  const accent = useColorModeValue("teal.400", "teal.300");

  const totalGenerations = selectedExams.size * generationCount;
  const hasProgress = generationProgress.size > 0;

  return (
    <Box minH="100vh" bg={bg} py={12} px={4}>
      <HStack position="fixed" top={6} left={8} zIndex={101} spacing={3}>
        <Button
          colorScheme="teal"
          variant="outline"
          size="sm"
          leftIcon={<ArrowLeft size={16} />}
          onClick={() => navigate({ to: landingRoute.to })}
        >
          Back to Dashboard
        </Button>
        <Button
          colorScheme="red"
          variant="outline"
          size="sm"
          onClick={() => logout()}
        >
          Logout
        </Button>
      </HStack>
      <Center>
        <Stack spacing={8} w="full" maxW="5xl">
          <Flex
            justify="space-between"
            align="center"
            bg={cardBg}
            borderRadius="xl"
            p={8}
            boxShadow="lg"
            mb={4}
          >
            <Stack spacing={1}>
              <Heading color={accent} fontWeight="extrabold" fontSize="3xl">
                <HStack>
                  <Sparkles size={32} />
                  <Text>Exam Generations</Text>
                </HStack>
              </Heading>
              <Text color="gray.300" fontSize="lg">
                Generate multiple exam instances from your exam configurations.
              </Text>
            </Stack>
          </Flex>

          {/* Configuration Panel */}
          <Box bg={cardBg} borderRadius="xl" p={8} boxShadow="lg">
            <Stack spacing={6}>
              <Heading size="md" color={accent}>
                Configuration
              </Heading>

              <FormControl>
                <FormLabel color="gray.300">
                  Number of Generations per Exam
                </FormLabel>
                <NumberInput
                  value={generationCount}
                  onChange={(_, valueAsNumber) =>
                    setGenerationCount(valueAsNumber)
                  }
                  min={1}
                  max={100}
                  isDisabled={isGenerating}
                >
                  <NumberInputField />
                  <NumberInputStepper>
                    <NumberIncrementStepper />
                    <NumberDecrementStepper />
                  </NumberInputStepper>
                </NumberInput>
                <FormHelperText color="gray.400">
                  Each selected exam will generate {generationCount} instance
                  {generationCount !== 1 ? "s" : ""}
                </FormHelperText>
              </FormControl>

              <Box>
                <Text color="gray.300" fontWeight="bold" mb={3}>
                  Select Exams
                </Text>
                <HStack spacing={4} mb={4}>
                  <Button
                    size="sm"
                    variant="outline"
                    colorScheme="teal"
                    onClick={handleSelectAll}
                    isDisabled={!examsQuery.data || isGenerating}
                  >
                    Select All
                  </Button>
                  <Button
                    size="sm"
                    variant="outline"
                    colorScheme="red"
                    onClick={handleDeselectAll}
                    isDisabled={selectedExams.size === 0 || isGenerating}
                  >
                    Deselect All
                  </Button>
                  <Text color="gray.400" fontSize="sm">
                    {selectedExams.size} exam{selectedExams.size !== 1 ? "s" : ""}{" "}
                    selected
                  </Text>
                </HStack>

                {examsQuery.isPending ? (
                  <Center py={8}>
                    <Spinner color={accent} size="lg" />
                  </Center>
                ) : examsQuery.isError ? (
                  <Alert status="error" borderRadius="md">
                    <AlertIcon />
                    <AlertTitle>Error loading exams</AlertTitle>
                    <AlertDescription>
                      {examsQuery.error.message}
                    </AlertDescription>
                  </Alert>
                ) : (
                  <SimpleGrid columns={{ base: 1, md: 2, lg: 3 }} spacing={4}>
                    {examsQuery.data.map(({ exam }) => (
                      <Card
                        key={exam.id}
                        bg={
                          selectedExams.has(exam.id) ? "teal.900" : "gray.700"
                        }
                        borderWidth={2}
                        borderColor={
                          selectedExams.has(exam.id) ? "teal.400" : "gray.600"
                        }
                        cursor="pointer"
                        onClick={() => handleExamSelection(exam.id)}
                        _hover={{
                          borderColor: "teal.300",
                          transform: "translateY(-2px)",
                        }}
                        transition="all 0.2s"
                      >
                        <CardBody>
                          <VStack align="start" spacing={2}>
                            <Text
                              fontWeight="bold"
                              color="white"
                              fontSize="md"
                              noOfLines={2}
                            >
                              {exam.config.name || "Untitled Exam"}
                            </Text>
                            <Text color="gray.400" fontSize="sm" noOfLines={1}>
                              {exam.config.questionSets.length} question set
                              {exam.config.questionSets.length !== 1 ? "s" : ""}
                            </Text>
                          </VStack>
                        </CardBody>
                      </Card>
                    ))}
                  </SimpleGrid>
                )}
              </Box>

              <Flex justify="space-between" align="center" pt={4}>
                <VStack align="start" spacing={1}>
                  <Text color="gray.300" fontWeight="bold">
                    Total Generations: {totalGenerations}
                  </Text>
                  <Text color="gray.400" fontSize="sm">
                    {selectedExams.size} exam{selectedExams.size !== 1 ? "s" : ""}{" "}
                    × {generationCount} generation
                    {generationCount !== 1 ? "s" : ""}
                  </Text>
                </VStack>
                <HStack spacing={4}>
                  {hasProgress && (
                    <Button
                      colorScheme="gray"
                      variant="outline"
                      onClick={handleReset}
                      isDisabled={isGenerating}
                    >
                      Reset
                    </Button>
                  )}
                  <Button
                    leftIcon={<Sparkles size={18} />}
                    colorScheme="teal"
                    size="lg"
                    onClick={handleGenerateExams}
                    isDisabled={
                      selectedExams.size === 0 ||
                      generationCount < 1 ||
                      isGenerating
                    }
                    isLoading={isGenerating}
                    loadingText="Generating..."
                  >
                    Generate Exams
                  </Button>
                </HStack>
              </Flex>
            </Stack>
          </Box>

          {/* Progress Panel */}
          {hasProgress && (
            <Box bg={cardBg} borderRadius="xl" p={8} boxShadow="lg">
              <Stack spacing={6}>
                <Heading size="md" color={accent}>
                  Generation Progress
                </Heading>

                {Array.from(generationProgress.values()).map((progress) => (
                  <Card key={progress.examId} bg="gray.700">
                    <CardHeader pb={2}>
                      <Flex justify="space-between" align="center">
                        <Text fontWeight="bold" color="white">
                          {progress.examName}
                        </Text>
                        <Badge
                          colorScheme={
                            progress.status === "completed"
                              ? "green"
                              : progress.status === "failed"
                              ? "red"
                              : progress.status === "in-progress"
                              ? "blue"
                              : "gray"
                          }
                        >
                          {progress.status}
                        </Badge>
                      </Flex>
                    </CardHeader>
                    <CardBody pt={2}>
                      <VStack align="stretch" spacing={2}>
                        <Progress
                          value={(progress.completed / progress.total) * 100}
                          colorScheme={
                            progress.failed > 0 ? "yellow" : "teal"
                          }
                          size="sm"
                          borderRadius="md"
                        />
                        <Flex justify="space-between" fontSize="sm">
                          <Text color="gray.400">
                            {progress.completed} / {progress.total} completed
                          </Text>
                          {progress.failed > 0 && (
                            <Text color="red.400">
                              {progress.failed} failed
                            </Text>
                          )}
                        </Flex>
                        {progress.error && (
                          <Alert status="error" size="sm" borderRadius="md">
                            <AlertIcon />
                            <AlertDescription fontSize="sm">
                              {progress.error}
                            </AlertDescription>
                          </Alert>
                        )}
                      </VStack>
                    </CardBody>
                  </Card>
                ))}
              </Stack>
            </Box>
          )}
        </Stack>
      </Center>
    </Box>
  );
}

export const generationsRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/generations",
  component: () => (
    <ProtectedRoute>
      <Generations />
    </ProtectedRoute>
  ),
});
